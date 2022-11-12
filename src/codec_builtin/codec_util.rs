use std::pin::Pin;

use futures::Future;

use crate::protocol::client_message::{ClientMessage, Frame};

pub struct CodecUtil;

impl CodecUtil {
  pub async fn fast_forward_to_end_frame(client_message: &mut ClientMessage) {
    let mut number_of_expected_end_frames = 1;
    let mut frame: Option<Frame> = None;
    while number_of_expected_end_frames != 0 {
      frame = client_message.next_frame().await;
      if frame.as_ref().unwrap().is_end_frame().await {
        number_of_expected_end_frames -= 1;
      } else if frame.as_ref().unwrap().is_begin_frame().await {
        number_of_expected_end_frames += 1;
      }
    }
  }

  pub async fn encode_nullable<'a, T>(client_message: &'a mut ClientMessage, value: &'a Option<T>, encoder: impl Fn(&'a mut ClientMessage, &'a T) -> Pin<Box<dyn Future<Output=()> + Send + Sync + 'a>>) {
    if value.is_none() {
      client_message.add_frame(Frame::new_null_frame().copy()).await;
    } else {
      encoder(client_message, value.as_ref().unwrap()).await;
    }
  }

  pub fn decode_nullable<'a, T: Send + Sync>(
    client_message: &'a mut ClientMessage,
    decoder: impl Fn(&'a mut ClientMessage) -> Pin<Box<dyn Future<Output=T> + Send + Sync + 'a>> + Send + Sync + 'a,
  ) -> Pin<Box<dyn Future<Output=Option<T>> + Send + Sync + 'a>> {
    Box::pin(async move {
      if CodecUtil::next_frame_is_null_frame(client_message).await {
        None
      } else {
        Some(decoder(client_message).await)
      }
    })
  }

  pub async fn next_frame_is_data_structure_end_frame(client_message: &mut ClientMessage) -> bool {
    client_message.peek_next_frame().unwrap().is_end_frame().await
  }

  pub async fn next_frame_is_null_frame(client_message: &mut ClientMessage) -> bool {
    let is_null = client_message.peek_next_frame().unwrap().is_null_frame().await;
    if is_null {
      client_message.next_frame().await;
    }
    return is_null;
  }
}