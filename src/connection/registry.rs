use std::collections::HashMap;
use std::sync::Arc;
use rand::Rng;
use tokio::sync::RwLock;
use uuid::Uuid;
use crate::cluster::service::ClusterService;
use crate::config::connection::ReconnectMode;
use crate::connection::registry::ClientState::{Initial, InitializedOnCluster};
use crate::network::connection::Connection;

#[derive(Eq, PartialEq)]
#[repr(u8)]
pub enum ClientState {
  Initial = 0,
  ConnectedToCluster = 1,
  InitializedOnCluster = 2,
}

pub struct ConnectionRegistry {
  async_start: bool,
  pub reconnect_mode: RwLock<ReconnectMode>,
  smart_routing_enabled: bool,
  cluster_service: Arc<ClusterService>,
  active_connections: RwLock<HashMap<String, Connection>>,
  pub client_state: RwLock<ClientState>,
}

impl ConnectionRegistry {
  pub fn new(
    async_start: bool,
    reconnect_mode: ReconnectMode,
    smart_routing_enabled: bool,
    cluster_service: Arc<ClusterService>,
  ) -> Self {
    ConnectionRegistry {
      async_start,
      reconnect_mode: RwLock::new(reconnect_mode),
      smart_routing_enabled,
      cluster_service,
      active_connections: RwLock::new(HashMap::new()),
      client_state: RwLock::new(ClientState::Initial),
    }
  }

  pub async fn get_connection(&self, member_uuid: Option<Uuid>) -> Option<Connection> {
    if let Some(member_uuid) = member_uuid {
      self.active_connections.read().await.get(&member_uuid.to_string()).cloned()
    } else {
      None
    }
  }

  pub async fn delete_connection(&self, member_uuid: Uuid) {
    self.active_connections.write().await.remove(&member_uuid.to_string());
  }

  pub async fn get_random_connection(&self) -> Option<Connection> {
    //todo: Add smart routing

    let active_connections = self.active_connections.read().await;
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..active_connections.len());
    active_connections.values().nth(random_index).cloned()
  }

  pub async fn is_empty(&self) -> bool {
    self.active_connections.read().await.is_empty()
  }

  pub async fn set_connection(&self, member_uuid: Uuid, connection: Connection) {
    self.active_connections.write().await.insert(member_uuid.to_string(), connection);
  }

  pub async fn set_client_state(&self, client_state: ClientState) {
    *self.client_state.write().await = client_state;
  }

  pub async fn get_connections(&self) -> HashMap<String, Connection> {
    self.active_connections.read().await.clone()
  }

  pub async fn check_if_invocation_allowed(&self) -> Option<String> {
    let state = self.client_state.read().await;
    if &*state == &InitializedOnCluster && self.active_connections.read().await.len() > 0 {
      return None;
    }
    let error = if &*state == &Initial {
      if self.async_start {
        "Client is not active yet".into()
      } else {
        "No connection found to cluster since the client is starting.".into()
      }
    } else if *self.reconnect_mode.read().await == ReconnectMode::Async {
      "Client is not active yet".into()
    } else {
      "No connection found to cluster.".into()
    };
    Some(error)
  }
}