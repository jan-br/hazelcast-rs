use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::invocation::connection_registration::ConnectionRegistration;
use crate::invocation::InvocationReturnValue;
use crate::listener::message_codec::ListenerMessageCodec;
use crate::network::connection::Connection;
use crate::protocol::client_message::ClientMessage;

pub struct ListenerRegistration {
  pub connection_registrations: Arc<RwLock<HashMap<i32, ConnectionRegistration>>>,
  pub handler: Arc<Pin<Box<dyn Send + Sync + Fn(ClientMessage) -> Pin<Box<dyn Send + Sync + Future<Output=()>>>>>>,
  pub codec: Pin<Box<dyn ListenerMessageCodec + Send + Sync>>,
}

impl ListenerRegistration {
  pub fn new(handler: impl Send + Sync + Fn(ClientMessage) -> Pin<Box<dyn Send + Sync + Future<Output=()>>> + 'static, codec: impl ListenerMessageCodec + Send + Sync + 'static) -> Self {
    Self {
      connection_registrations: Arc::new(RwLock::new(HashMap::new())),
      handler: Arc::new(Box::pin(handler)),
      codec: Box::pin(codec),
    }
  }
}