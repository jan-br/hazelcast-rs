use std::future::Future;
use std::pin::Pin;
use uuid::Uuid;
use crate::protocol::client_message::ClientMessage;

pub trait ListenerMessageCodec {
  fn encode_add_request<'a>(&'a self, local_only: &'a bool) -> Pin<Box<dyn Future<Output=ClientMessage> + Send + Sync + 'a>>;
  fn decode_add_response<'a>(&'a self, client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=Uuid> + Send + Sync + 'a>>;
  fn encode_remove_request<'a>(&'a self, registration_id: &'a Uuid) -> Pin<Box<dyn Future<Output=ClientMessage> + Send + Sync + 'a>>;
}