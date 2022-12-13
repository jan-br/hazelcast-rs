use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use async_recursion::async_recursion;
use event_listener_primitives::Bag;

use futures::{FutureExt, join, StreamExt};
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpStream;
use tokio::sync::RwLock;
use tokio::time::timeout;
use uuid::Uuid;
use tokio::sync::Mutex;

use crate::cluster::candidate::CandidateClusterContext;
use crate::cluster::failover::ClusterFailoverService;
use crate::cluster::service::ClusterService;
use crate::codec::client_authentication_codec::{ClientAuthenticationCodec, ClientAuthenticationResponseParams};
use crate::config::ClientConfig;
use crate::config::connection::ReconnectMode;
use crate::connection::address::Address;
use crate::connection::registry::{ClientState, ConnectionRegistry};
use crate::core::member::Member;
use crate::DefaultAddressProvider;
use crate::invocation::Invocation;
use crate::invocation::service::InvocationService;
use crate::lifecycle_service::{LifecycleService, LifecycleState};
use crate::network::connection::Connection;
use crate::network::heartbeat_manager::HeartbeatManager;
use crate::network::wait_strategy::WaitStrategy;
use crate::partition_service::PartitionService;
use crate::protocol::authentication_status::AuthenticationStatus;
use crate::protocol::client_message::ClientMessage;
use crate::util::future::DeferredFuture;

pub struct ConnectionManager {
  pub config: Arc<ClientConfig>,
  pub cluster_failover_service: Arc<ClusterFailoverService>,
  pub switching_to_next_cluster: RwLock<bool>,
  pub wait_strategy: RwLock<WaitStrategy>,
  pub cluster_service: Arc<ClusterService>,
  pub connection_registry: Arc<ConnectionRegistry>,
  pub cluster_id: RwLock<Option<Uuid>>,
  pub pending_connections: RwLock<HashMap<String, DeferredFuture<Connection, ()>>>,
  pub invocation_service: Arc<InvocationService>,
  pub heartbeat_manager: Arc<HeartbeatManager>,
  pub client_uuid: Uuid,
  pub partition_service: Arc<PartitionService>,
  pub lifecycle_service: Arc<LifecycleService>,
  pub connection_added_bag: RwLock<Bag<Arc<dyn Fn(&Connection) + Send + Sync>, Connection>>,
  pub connection_removed_bag: RwLock<Bag<Arc<dyn Fn(&Connection) + Send + Sync>, Connection>>,
  pub reconnect_mode: RwLock<ReconnectMode>,
  pub connect_to_cluster_task_submitted: RwLock<bool>,
}

impl ConnectionManager {
  pub const CLIENT_TYPE: &'static str = "RS";
  pub const SERIALIZATION_VERSION: u8 = 1;
  pub const SET_TIMEOUT_MAX_DELAY: i32 = 2147483647;
  pub const BINARY_PROTOCOL_VERSION: &'static [u8; 3] = b"CP2";

  pub async fn new(
    config: Arc<ClientConfig>,
    cluster_failover_service: Arc<ClusterFailoverService>,
    cluster_service: Arc<ClusterService>,
    connection_registry: Arc<ConnectionRegistry>,
    invocation_service: Arc<InvocationService>,
    partition_service: Arc<PartitionService>,
    lifecycle_service: Arc<LifecycleService>,
  ) -> Self {
    let retry = config.retry.read().await;

    let wait_strategy = RwLock::new(WaitStrategy::new(
      retry.initial_backoff,
      retry.max_backoff,
      retry.multiplier,
      retry.jitter,
      retry.cluster_connect_timeout,
    ));
    drop(retry);

    let heartbeat_manager = Arc::new(HeartbeatManager::new());

    ConnectionManager {
      lifecycle_service,
      connection_added_bag: RwLock::new(Bag::default()),
      connection_removed_bag: RwLock::new(Bag::default()),
      cluster_id: RwLock::new(None),
      cluster_failover_service,
      switching_to_next_cluster: RwLock::new(false),
      wait_strategy,
      config,
      cluster_service,
      reconnect_mode: RwLock::new(connection_registry.clone().reconnect_mode.read().await.clone()),
      connection_registry,
      pending_connections: RwLock::new(HashMap::new()),
      client_uuid: Uuid::new_v4(),
      invocation_service,
      heartbeat_manager,
      partition_service,
      connect_to_cluster_task_submitted: RwLock::new(false),
    }
  }

  pub async fn connect_to_cluster(
    self: &Arc<ConnectionManager>,
  ) {
    self.do_connect_to_cluster().await;
  }

  pub async fn do_connect_to_cluster(
    self: &Arc<ConnectionManager>,
  ) {
    let current_context = self.cluster_failover_service.current().await;
    self.do_connect_to_candidate_cluster(current_context)
      .then(|connected| async move {
        if connected {
          true
        } else {
          self.cluster_failover_service
            .try_next_cluster(|context| self.cleanup_and_try_next_cluster(context))
            .await
        }
      })
      .then(|connected| async move {
        if !connected {
          todo!("Unable to connect to any cluster")
        }
      })
      .await;
  }

  pub async fn cleanup_and_try_next_cluster(
    self: &Arc<ConnectionManager>,
    next_context: Arc<CandidateClusterContext>,
  ) -> bool {
    //todo: notify client on cluster change
    let mut switching_to_next_cluster = self.switching_to_next_cluster.write().await;
    *switching_to_next_cluster = true;
    self.do_connect_to_candidate_cluster(next_context)
      .then(|connected| async move {
        if connected {
          todo!()
        } else {
          false
        }
      })
      .await
  }

  pub async fn do_connect_to_candidate_cluster(
    self: &Arc<ConnectionManager>,
    context: Arc<CandidateClusterContext>,
  ) -> bool {
    let tried_addresses = vec![];
    self.wait_strategy.write().await.reset();
    self.try_connecting_to_addresses(context, tried_addresses)
      .await
  }

  pub async fn try_connecting_to_addresses(
    self: &Arc<ConnectionManager>,
    context: Arc<CandidateClusterContext>,
    mut tried_addresses: Vec<String>,
  ) -> bool {
    loop {
      let mut tried_addresses_per_attempt = vec![];
      let tried_addresses_per_attempt = &mut tried_addresses_per_attempt;

      let members = self.cluster_service.get_members(None).await;
      let connected = self
        .try_connecting(
          &members,
          tried_addresses_per_attempt,
          |m| m.address.clone(),
          |m| self.get_or_connect_to_member(m),
        )
        .await;

      let connected = if connected {
        true
      } else {
        let addresses = self
          .load_addresses_from_provider(context.address_provider.clone())
          .await
          .into_iter()
          .filter(|address| !tried_addresses_per_attempt.contains(&address.to_string()))
          .collect::<Vec<_>>();
        self.try_connecting(
          &addresses,
          tried_addresses_per_attempt,
          |a| (*a).clone(),
          |a| self.get_or_connect_to_address(a),
        )
          .await
      };
      if connected {
        return true;
      }
      for address in tried_addresses_per_attempt.iter() {
        tried_addresses.push(address.clone());
      }
      //todo: check for cluster shutdown
      let mut wait_strategy = self.wait_strategy.write().await;
      let not_timed_out = wait_strategy.sleep().await;
      if !not_timed_out {
        return false;
      }
    }
  }

  pub fn get_or_connect_to_address<'a>(
    self: &'a Arc<ConnectionManager>,
    address: Arc<Address>,
  ) -> Pin<Box<dyn Future<Output=Option<Connection>> + Send + Sync + 'a>> {
    Box::pin(async move {
      //todo: add shutdown check

      let connection = self.get_connection_for_address(address.clone()).await;
      if connection.is_some() {
        return connection;
      }
      // self.get_or_connect(address.clone(), || self.translate_address(address))
      //   .await
      todo!()
    })
  }

  pub fn translate_address<'a>(&'a self, address: Arc<Address>) -> Pin<Box<dyn Future<Output=Option<Arc<Address>>> + Send + Sync + 'a>> {
    Box::pin(async move {
      let current = self.cluster_failover_service.current().await;
      let address_provider = current.address_provider.clone();
      address_provider.translate(address)
    })
  }

  pub fn get_or_connect<'a>(
    self: &'a Arc<ConnectionManager>,
    address: Arc<Address>,
    translate_address_fn: impl FnOnce() -> Pin<Box<dyn Future<Output=Option<Arc<Address>>> + Send + Sync + 'a>> + Send + Sync + 'static,
  ) -> Pin<Box<dyn Future<Output=Option<Connection>> + Send + Sync + 'a>> {
    Box::pin(async move {
      let address_key = address.to_string();
      let mut pending_connections = self.pending_connections.write().await;

      if let Some(pending_connection) = pending_connections.get_mut(&address_key) {
        return Some(pending_connection.wait().await.unwrap());
      }

      let mut connection_resolver = DeferredFuture::default();
      pending_connections.insert(address_key.clone(), connection_resolver.clone());

      let (_, (connection)) = join!({
        let mut connection_resolver = connection_resolver.clone();
        async move {
        let translated = translate_address_fn().await;
        if translated.is_none() {
            todo!("Unable to translate address");
        }

        let receiver = self.trigger_connect(translated.clone().unwrap());
        let translated_address = translated.unwrap();
        let tcp_stream = receiver.await.unwrap().unwrap();
        let (read_half, mut write_half) = tcp_stream.into_split();
        self.initiate_communication(&mut write_half).await;

        lazy_static::lazy_static! {
          static ref CONNECTION_ID: Mutex<i32> = Mutex::new(0);
        }
        let mut id = CONNECTION_ID.lock().await;
        *id+=1;
        let connection = Connection::new(translated_address, write_half, read_half, *id, self.clone());

        tokio::spawn({
          let connection = connection.clone();
          async move {
            connection.start_reader().await;
          }
        });
        connection
            .set_read_callback({
              let invocation_service = self.invocation_service.clone();
              Box::pin(move |response| Box::pin({
                let invocation_service = invocation_service.clone();
                async move {
                  let response = response.clone();
                  invocation_service.process_response(response).await;
                }
              }))
            })
            .await;

        let connection = self.authenticate_on_cluster(connection).await;
        println!("Authenticated on cluster");
        connection_resolver.resolve(connection).await;


      }}, async move {
        let connection = connection_resolver.wait().await.unwrap();
        pending_connections.remove(&address_key);
        connection
      });


      // let (_, (connection)) = join!(translate_address_fn()
      //   .then(|translated| async move {
      //     if translated.is_none() {
      //       todo!("Unable to translate address");
      //     }
      //     (
      //       self.trigger_connect(translated.clone().unwrap()).await.lock().await,
      //       translated.unwrap(),
      //     )
      //   })
      //   .then({
      //    let mut connection_resolver = connection_resolver.clone();
      //     let this = self.clone();
      //    move |(receiver, translated_address)| {
      //     let mut connection_resolver = connection_resolver.clone();
      //     async move {
      //       let tcp_stream: TcpStream = receiver.await.unwrap().unwrap();
      //       let (read_half, mut write_half) = tcp_stream.into_split();
      //       self.initiate_communication(&mut write_half).await;
      //
      //       lazy_static::lazy_static! {
      //         static ref CONNECTION_ID: Mutex<i32> = Mutex::new(0);
      //       }
      //       let mut id = CONNECTION_ID.lock().await;
      //       *id+=1;
      //       let connection = Connection::new(translated_address, write_half, read_half, *id, this);
      //
      //       tokio::spawn({
      //         let connection = connection.clone();
      //         async move {
      //           connection.start_reader().await;
      //         }
      //       });
      //       connection
      //           .set_read_callback({
      //             let invocation_service = self.invocation_service.clone();
      //             Box::pin(move |response| Box::pin({
      //               let invocation_service = invocation_service.clone();
      //               async move {
      //                 let response = response.clone();
      //                 invocation_service.process_response(response).await;
      //               }
      //             }))
      //           })
      //           .await;
      //
      //       let connection = self.authenticate_on_cluster(connection).await;
      //       println!("Authenticated on cluster");
      //       connection_resolver.resolve(connection).await;
      //     }
      //   }
      // }), async move {
      //   let connection = connection_resolver.wait().await.unwrap();
      //   pending_connections.remove(&address_key);
      //   connection
      // });


      Some(connection)
    })
  }

  pub async fn authenticate_on_cluster(&self, connection: Connection) -> Connection {
    let request = self.encode_authentication_request().await;
    let mut invocation = Invocation::new(self.invocation_service.clone(), request);
    invocation.connection = Some(connection.clone());
    invocation.handler = Some(Box::pin(|mut client_message| Box::pin(async move {
      Box::new(Box::new(ClientAuthenticationCodec::decode_response(&mut client_message).await))
    })));
    let response = timeout(
      self.heartbeat_manager
        .hartbeat_timeout
        .clone()
        .to_std()
        .unwrap(),
      self.invocation_service
        .invoke_urgent(&self.connection_registry, invocation),
    )
      .await
      .unwrap();

    let authentication_status = response.status;

    if authentication_status == AuthenticationStatus::Authenticated as u8 {
      self.on_authenticated(connection, response).await
    } else {
      todo!("Authentication failed")
    }
  }

  pub async fn check_partition_count(&self, partition_count: i32) {
    if !self.partition_service.check_and_set_partition_count(partition_count).await {
      todo!();
    }
  }

  pub async fn on_authenticated(&self, connection: Connection, response: ClientAuthenticationResponseParams) -> Connection {
    self.check_partition_count(response.partition_count).await;
    connection.set_connected_server_version(response.server_hazelcast_version).await;
    connection.set_remote_address(response.address).await;
    connection.set_remote_uuid(response.member_uuid).await;

    if let Some(existing_connection) = self.connection_registry.get_connection(response.member_uuid).await {
      existing_connection.close(format!("Duplicate connection to same member with uuid: {}", response.member_uuid.unwrap().to_string()).to_string(), None).await;
      return existing_connection;
    }

    let mut cluster_id = self.cluster_id.write().await;
    let new_cluster_id = response.cluster_id;
    let cluster_id_changed = cluster_id.is_some() && new_cluster_id != cluster_id.unwrap();

    if cluster_id_changed {
      todo!("new cluster id");
    }

    let connections_empty = self.connection_registry.is_empty().await;
    self.connection_registry.set_connection(response.member_uuid.unwrap(), connection.clone()).await;

    if connections_empty {
      *cluster_id = Some(new_cluster_id);
      if cluster_id_changed {
        self.connection_registry.set_client_state(ClientState::ConnectedToCluster).await;
        self.initialize_client_on_cluster(new_cluster_id).await;
      } else {
        self.connection_registry.set_client_state(ClientState::InitializedOnCluster).await;
        self.emit_lifecycle_event(LifecycleState::Connected).await;
      }
    }

    //todo: log
    self.emit_connection_added_event(&connection).await;
    connection
  }

  pub async fn emit_connection_removed_event(&self, connection: &Connection) {
    self.connection_removed_bag.read().await.call_simple(connection);
  }

  pub async fn emit_connection_added_event(&self, connection: &Connection) {
    self.connection_added_bag.read().await.call_simple(connection);
  }

  pub async fn emit_lifecycle_event(&self, state: LifecycleState) {
    self.lifecycle_service.emit_lifecycle_event(state).await;
  }

  pub async fn initialize_client_on_cluster(&self, new_cluster_id: Uuid) {
    todo!()
  }

  pub async fn encode_authentication_request(&self) -> ClientMessage {
    let context = self.cluster_failover_service.current().await;
    let cluster_name = context.config.cluster_name.clone();
    let security = context.config.security.read().await;

    //todo: Implement custom credentials

    let username = &security.username;
    let password = &security.password;
    let client_uuid = &self.client_uuid;
    let client_name = &self.config.client_name;

    ClientAuthenticationCodec::encode_request(
      &cluster_name.to_string(),
      &username.as_ref(),
      &password.as_ref(),
      &Some(&client_uuid),
      &Self::CLIENT_TYPE.to_string(),
      &Self::SERIALIZATION_VERSION,
      &"1.0.0".to_string(),
      client_name,
      &vec![],
    )
      .await
  }

  pub async fn initiate_communication(&self, stream: &mut OwnedWriteHalf) {
    stream.write_all(b"CP2").await.unwrap();
  }

  pub fn trigger_connect(
    &self,
    translated_address: Arc<Address>,
  ) -> tokio::sync::oneshot::Receiver<Option<TcpStream>> {
    //todo: add ssl handling
    self.connect_net_socket(translated_address)
  }

  pub fn connect_net_socket(
    &self,
    translated_addres: Arc<Address>,
  ) -> tokio::sync::oneshot::Receiver<Option<TcpStream>> {
    let (sender, receiver) = tokio::sync::oneshot::channel();
    tokio::spawn({
      async move {
        match TcpStream::connect((translated_addres.host.clone(), translated_addres.port as u16))
          .await
        {
          Ok(tcp_stream) => {
            sender.send(Some(tcp_stream)).unwrap();
          }
          Err(_) => {
            sender.send(None).unwrap();
          }
        };
      }
    });
    receiver
  }

  pub async fn get_connection_for_address(
    &self,
    address: Arc<Address>,
  ) -> Option<Connection> {
    for connection in self.connection_registry.get_connections().await.values() {
      if *connection.remote_address.lock().await == Some(address.clone()) {
        return Some(connection.clone());
      }
    }
    None
  }

  pub async fn load_addresses_from_provider(
    &self,
    address_provider: Arc<DefaultAddressProvider>,
  ) -> Vec<Arc<Address>> {
    let addressess = address_provider.load_addresses().await;
    let mut result = vec![];
    result.extend(addressess.primary);
    result.extend(addressess.secondary);
    result
  }

  pub fn get_or_connect_to_member(
    &self,
    member: Arc<Member>,
  ) -> Pin<Box<dyn Future<Output=Option<Connection>> + Send + Sync>> {
    todo!()
  }

  pub async fn try_connecting<'a, T: Send + Sync + ConnectingItem>(
    &'a self,
    items: &Vec<Arc<T>>,
    tried_addresses: &mut Vec<String>,
    get_address_fn: impl Fn(Arc<T>) -> Address,
    connect_to_fn: impl Fn(Arc<T>) -> Pin<Box<dyn Future<Output=Option<Connection>> + Send + Sync + 'a>> + Send + Sync,
  ) -> bool {
    for i in 0..items.len() {
      let item = &items[i];
      let address = get_address_fn(item.clone());
      tried_addresses.push(address.to_string());
      let connection = self
        .connect(item.clone(), || connect_to_fn(item.clone()))
        .await;
      if connection.is_some() {
        return true;
      }
      // } else {
      //     self.try_connecting(
      //         index + 1,
      //         items,
      //         tried_addresses,
      //         get_address_fn,
      //         connect_to_fn,
      //     )
      //     .await
      // }
    }
    false

    // if index >= items.len() {
    //     return false;
    // }
    // //todo: check if cluster is shutting down
    // let item = &items[index];
    // let address = get_address_fn(item.clone());
    // tried_addresses.push(address.to_string());
    //
    // let connection = self
    //     .connect(item.clone(), || connect_to_fn(item.clone()))
    //     .await;
    // if connection.is_some() {
    //     true
    // } else {
    //     self.try_connecting(
    //         index + 1,
    //         items,
    //         tried_addresses,
    //         get_address_fn,
    //         connect_to_fn,
    //     )
    //     .await
    // }
  }

  pub fn on_connection_close<'a>(
    self: &'a Arc<ConnectionManager>,
    connection: &'a Connection,
  ) -> Pin<Box<dyn Future<Output=()> + Send + Sync + 'a>> {
    Box::pin(async move {
      let endpoint = connection.remote_address.lock().await.clone();
      let member_uuid = connection.remote_uuid.lock().await.clone();

      if endpoint.is_none() {
        todo!()
      }

      let active_connection = self.connection_registry.get_connection(member_uuid).await;

      if let Some(active_connection) = active_connection {
        if connection.connection_id == active_connection.connection_id {
          self.connection_registry.delete_connection(member_uuid.unwrap()).await;
          if self.connection_registry.is_empty().await {
            let client_state = self.connection_registry.client_state.write().await;
            if *client_state == ClientState::InitializedOnCluster {
              self.emit_lifecycle_event(LifecycleState::Disconnected).await;
            }
            self.trigger_cluster_reconnection().await;
          }
          self.emit_connection_removed_event(connection).await;
        } else {
          todo!()
        }
      } else {
        todo!()
      }
    })
  }

  pub fn trigger_cluster_reconnection<'a>(
    self: &'a Arc<ConnectionManager>,
  ) -> Pin<Box<dyn Future<Output=()> + Send + Sync + 'a>> {
    Box::pin(async move {
      if *self.reconnect_mode.read().await == ReconnectMode::Off {
        todo!()
      }

      if self.lifecycle_service.is_running().await {
        self.submit_connect_to_cluster_task().await;
      }
    })
  }

  fn submit_connect_to_cluster_task<'a>(
    self: &'a Arc<ConnectionManager>,
  ) -> Pin<Box<dyn Future<Output=()> + Send + Sync + 'a>> {
    Box::pin(async move {
      if *self.connect_to_cluster_task_submitted.read().await {
        return;
      }

      *self.connect_to_cluster_task_submitted.write().await = true;
      self.do_connect_to_cluster().await;
      if self.connection_registry.is_empty().await {
        self.submit_connect_to_cluster_task().await;
      }
      *self.connect_to_cluster_task_submitted.write().await = false;
    })
  }

  pub async fn connect<'a>(
    &'a self,
    target: Arc<impl ConnectingItem>,
    get_or_connect_fn: impl Fn() -> Pin<Box<dyn Future<Output=Option<Connection>> + Send + Sync + 'a>> + Send + Sync,
  ) -> Option<Connection> {
    //todo: add error handling
    get_or_connect_fn().await
  }
}

pub trait ConnectingItem {}

impl ConnectingItem for Member {}

impl ConnectingItem for Address {}
