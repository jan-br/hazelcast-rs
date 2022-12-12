use std::collections::HashMap;
use std::future::Future;
use std::mem::transmute;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task::LocalSet;
use uuid::Uuid;
use crate::connection::manager::ConnectionManager;
use crate::invocation::{Invocation, InvocationReturnValue};
use crate::invocation::listener_registration::ListenerRegistration;
use crate::invocation::service::InvocationService;
use crate::listener::message_codec::ListenerMessageCodec;
use crate::network::connection::Connection;
use crate::protocol::client_message::ClientMessage;

pub struct ListenerService {
  invocation_service: Arc<InvocationService>,
  registrations: Arc<RwLock<HashMap<Uuid, Arc<ListenerRegistration>>>>,
  connection_manager: Arc<ConnectionManager>,
  is_smart_service: bool,
}

impl ListenerService {
  pub fn new(invocation_service: Arc<InvocationService>, connection_manager: Arc<ConnectionManager>) -> Self {
    Self {
      registrations: Arc::new(RwLock::new(HashMap::new())),
      connection_manager,
      invocation_service,
      is_smart_service: false, //make smart
    }
  }

  pub async fn register_listener(self: &Arc<Self>, codec: impl ListenerMessageCodec + Send + Sync + 'static, handler: impl Send + Sync + Fn(ClientMessage) -> Pin<Box<dyn Send + Sync + Future<Output=()>>> + 'static) {
    let user_registration_id = Uuid::new_v4();

    let listener_registration = Arc::new(ListenerRegistration::new(handler, codec));
    self.registrations.write().await.insert(user_registration_id, unsafe { transmute(listener_registration.clone()) });

    let active_connections = self.connection_manager.connection_registry.get_connections().await;

    let set = LocalSet::new();
    for connection in active_connections.into_values() {
      let this = self.clone();
      let listener_registration = listener_registration.clone();
      set.spawn_local(async move {
        this.invoke(listener_registration, connection, user_registration_id).await;
      });
    }
    set.await;
  }

  async fn invoke(&self, listener_registration: Arc<ListenerRegistration>, connection: Connection, user_registration_id: Uuid) {
    let connection_registrations = listener_registration.connection_registrations.write().await;
    if connection_registrations.contains_key(&connection.connection_id) {
      return;
    }

    let register_request = listener_registration.codec.encode_add_request(&self.is_smart_service).await;
    //todo: Add logging

    let mut invocation: Invocation<Box<Box<Uuid>>> = Invocation::new(self.invocation_service.clone(), register_request);
    invocation.handler = Some(Box::pin({
      let listener_registration = listener_registration.clone();
      move |mut client_message| Box::pin({
        let listener_registration = listener_registration.clone();
        async move {
          Box::new(Box::new(listener_registration.codec.decode_add_response(&mut client_message).await))
        }
      })
    }));
    invocation.event_handler = Some(listener_registration.handler.clone());
    invocation.connection = Some(connection);

    let uuid = self.invocation_service.invoke_urgent(&self.connection_manager.connection_registry, invocation).await;
    println!("ugh: {:?}", uuid);
  }
}