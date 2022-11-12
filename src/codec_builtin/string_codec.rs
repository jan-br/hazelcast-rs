use std::future::Future;
use std::pin::Pin;
use crate::protocol::client_message::{ClientMessage, Frame};

pub struct StringCodec;

impl StringCodec {
  pub fn encode<'a>(client_message: &'a mut ClientMessage, value: &'a (impl ToString + Send + Sync)) -> Pin<Box<dyn Future<Output=()> + Send + Sync + 'a>> {
    Box::pin(async move {
      client_message.add_frame(Frame::new_default_flags(value.to_string().into_bytes())).await;
    })
  }

  pub fn decode<'a>(client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=String> + Send + Sync + 'a>> {
    Box::pin(async move {
      let frame = client_message.next_frame().await.unwrap();
      let bytes = frame.content.lock().await;
      String::from_utf8(bytes.clone()).unwrap()
    })
  }
}