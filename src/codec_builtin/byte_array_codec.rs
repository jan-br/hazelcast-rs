use crate::protocol::client_message::{ClientMessage, Frame};

pub struct ByteArrayCodec;

impl ByteArrayCodec {
  pub async fn encode(client_message: &mut ClientMessage, value: Vec<u8>) {
    client_message.add_frame(Frame::new_default_flags(value)).await;
  }

  pub async fn decode(client_message: &mut ClientMessage) -> Vec<u8> {
    let frame = client_message.next_frame().await.unwrap();
    let content = frame.content.lock().await;
    content.clone()
  }
}