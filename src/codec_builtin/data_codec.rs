use std::future::Future;
use std::pin::Pin;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::serialization::heap_data::HeapData;

pub struct DataCodec {}

impl DataCodec {
  pub fn encode<'a>(client_message: &'a mut ClientMessage, data: &'a HeapData) -> Pin<Box<dyn Future<Output=()> + Send + Sync + 'a>> {
    Box::pin(async move {
      client_message.add_frame(Frame::new_default_flags(data.to_buffer())).await;
    })
  }

  pub fn decode<'a>(client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=HeapData> + Send + Sync + 'a>> {
    Box::pin(async move {
      HeapData::new(client_message.next_frame().await.unwrap().content.lock().await.clone())
    })
  }

}