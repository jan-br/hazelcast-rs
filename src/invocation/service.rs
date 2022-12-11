use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use std::mem::{size_of, transmute};
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;
use chrono::Duration;
use tokio::sync::{RwLock, RwLockWriteGuard};
use crate::ClientConfig;
use crate::codec_builtin::error_codec::ErrorCodec;
use crate::connection::registry::ConnectionRegistry;
use crate::core::distributed_object::DistributedObject;
use crate::invocation::{Invocation, InvocationReturnValue};
use crate::network::connection::Connection;
use crate::protocol::client_message::ClientMessage;
use crate::proxy::map_proxy::AnySend;
use crate::util::future::DeferredFuture;

pub struct InvocationService {
  pub config: Arc<ClientConfig>,
  pub invocation_timeout: Duration,
  pub correllation_counter: RwLock<u64>,
  pub invocations: RwLock<HashMap<u64, Arc<RwLock<Invocation<Box<Box<dyn AnySend>>>>>>>,
}

impl InvocationService {
  pub fn new(config: Arc<ClientConfig>) -> Self {
    Self {
      config,
      invocation_timeout: Duration::seconds(10),
      correllation_counter: RwLock::new(1),
      invocations: RwLock::new(HashMap::new()),
    }
  }

  pub async fn invoke_urgent<R: InvocationReturnValue + Clone>(&self, connection_registry: &ConnectionRegistry, mut invocation: Invocation<Box<Box<R>>>) -> R {
    invocation.urgent = true;
    self.invoke(connection_registry, invocation).await
  }

  pub async fn invoke<R: InvocationReturnValue + Clone>(&self, connection_registry: &ConnectionRegistry, mut invocation: Invocation<Box<Box<R>>>) -> R {
    invocation.deferred = Some(DeferredFuture::default());
    let mut correllation_counter = self.correllation_counter.write().await;
    *correllation_counter += 1;
    invocation.request.set_correlation_id(*correllation_counter).await;
    let deferred = invocation.deferred.clone();
    let invocation = Arc::new(RwLock::new(invocation));
    self.do_invoke(connection_registry, invocation).await;
    **deferred.unwrap().wait().await.unwrap()
  }

  pub async fn do_invoke<R: InvocationReturnValue + Clone>(&self, connection_registry: &ConnectionRegistry, invocation: Arc<RwLock<Invocation<Box<Box<R>>>>>) {
    //todo: implement smart mode
    self.invoke_non_smart(connection_registry, invocation).await;
  }

  pub async fn invoke_on_partition<R: InvocationReturnValue + Send + Sync + Clone>(self: &Arc<Self>, connection_registry: &ConnectionRegistry, request: ClientMessage, partition_id: i32, decoder: Pin<Box<dyn Send + Sync + Fn(ClientMessage) -> Pin<Box<dyn Send + Sync + Future<Output=Box<Box<R>>>>>>>) -> R {
    let mut invocation = Invocation::<Box<Box<R>>>::new(self.clone(), request);
    invocation.partition_id = partition_id;
    invocation.handler = Some(decoder);

    self.invoke(connection_registry, invocation).await
  }

  pub async fn invoke_on_random_target<T: DistributedObject>(self: &Arc<Self>, connection_registry: &ConnectionRegistry, request: ClientMessage, handler: Pin<Box<dyn Send + Sync + Fn(ClientMessage) -> Pin<Box<dyn Send + Sync + Future<Output=Box<Box<Arc<T>>>>>>>>) -> Box<Arc<T>> {
    let mut invocation = Invocation::<Box<Box<Arc<T>>>>::new(self.clone(), request);
    invocation.handler = Some(handler);
    Box::new(self.invoke(connection_registry, invocation).await)
  }

  pub async fn invoke_non_smart<R: InvocationReturnValue + Clone>(&self, connection_registry: &ConnectionRegistry, invocation: Arc<RwLock<Invocation<Box<Box<R>>>>>) {
    let connection = {
      let mut invocation = invocation.write().await;
      invocation.invoke_count += 1;
      if !invocation.urgent {
        let error = connection_registry.check_if_invocation_allowed().await;
        if let Some(error) = error {
          self.notify_error(&mut *invocation, error).await;
          return;
        }
      }
      invocation.connection.clone()
    };

    if let Some(connection) = connection {
      self.send(invocation, connection).await
    } else {
      self.invoke_on_random_connection(connection_registry, invocation).await
    };
    //todo: Error handling
  }

  pub async fn invoke_on_random_connection<R: InvocationReturnValue + Clone>(&self, connection_registry: &ConnectionRegistry, invocation: Arc<RwLock<Invocation<Box<Box<R>>>>>) {
    let connection = connection_registry.get_random_connection().await;
    match connection {
      None => {
        todo!()
      }
      Some(connection) => {
        self.send(invocation, connection).await;
      }
    }
  }

  pub async fn process_response(&self, client_message: ClientMessage) {
    let correlation_id = client_message.get_correlation_id().await;
    let start_frame = if let Some(start_frame) = &client_message.start_frame {
      start_frame
    } else {
      return;
    };
    let invocation = self.invocations.write().await.get(&correlation_id).cloned();
    if start_frame.has_event_flag().await || start_frame.has_backup_event_flag().await {
      if invocation.is_none() {
        //todo: Add retry logic for missing invocation handler
        todo!()
      }
      return;
    }

    if invocation.is_none() {
      //todo: Add proper logging
      return;
    }
    let invocation = invocation.unwrap();

    let message_type = client_message.get_message_type().await;
    if message_type == ErrorCodec::EXCEPTION_MESSAGE_TYPE {
      todo!("implement error handling")
    } else {
      invocation.write().await.notify(client_message).await;
    }
  }

  pub async fn send<R: InvocationReturnValue + Clone>(&self, invocation: Arc<RwLock<Invocation<Box<Box<R>>>>>, connection: Connection) {
    //todo check if connection is alive
    //todo implement backup_ack_to_client_enabled
    /*if self.backup_ack_to_client_enabled {
      invocation.request.start_frame.unwrap().add_flag(IS_BACKUP_AWARE_FLAG);
    }*/
    self.register_invocation(invocation.clone()).await;
    connection.write(invocation.clone()).await;
    let mut invocation = invocation.write().await;
    invocation.send_connection = Some(connection);
  }

  pub async fn deregister_invocation(&self, correlation_id: u64) {
    self.invocations.write().await.remove(&correlation_id);
  }

  pub async fn register_invocation<R: InvocationReturnValue + Clone>(&self, invocation: Arc<RwLock<Invocation<Box<Box<R>>>>>) {
    let correlation_id = {
      let mut invocation = invocation.write().await;
      let partition_id = invocation.partition_id;
      let message = &mut invocation.request;
      let correlation_id = message.get_correlation_id().await;
      if partition_id >= 0 {
        message.set_partition_id(partition_id).await;
      } else {
        message.set_partition_id(-1).await;
      }
      correlation_id
    };
    let mut invocations = self.invocations.write().await;


    invocations.insert(correlation_id, unsafe { std::mem::transmute(invocation) });
  }

  pub async fn notify_error<R: InvocationReturnValue + Clone>(&self, invocation: &mut Invocation<Box<R>>, error: String) {
    std::mem::replace(&mut invocation.deferred, None).unwrap().reject(error).await;
  }
}