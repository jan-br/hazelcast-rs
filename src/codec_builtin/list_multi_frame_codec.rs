use std::pin::Pin;

use futures::Future;

use crate::codec_builtin::codec_util::CodecUtil;
use crate::protocol::client_message::{ClientMessage, Frame};

pub struct ListMultiFrameCodec;

impl ListMultiFrameCodec {
  pub fn encode<'a, T: Send + Sync>(
    client_message: &'a mut ClientMessage,
    list: &'a Vec<T>,
    encoder: impl for<'b> Fn(&'b mut ClientMessage, &'b T) -> Pin<Box<dyn Future<Output=()> + Send + Sync + 'b>> + Send + Sync + 'a,
  ) -> Pin<Box<dyn Future<Output=()> + Send + Sync + 'a>> {
    Box::pin(async move {
      client_message.add_frame(Frame::new_begin_frame().copy()).await;
      let n = list.len();
      for i in 0..n {
        encoder(client_message, &list[i]).await;
      }
      client_message.add_frame(Frame::new_end_frame().copy()).await;
    })
  }

  pub fn encode_contains_nullable<'a, T>(
    client_message: &'a mut ClientMessage,
    list: &'a mut Vec<Option<T>>,
    encoder: impl for<'b> Fn(&'b mut ClientMessage, &'b mut T) -> Pin<Box<dyn Future<Output=()> + 'b>> + 'a,
  ) -> Pin<Box<dyn Future<Output=()> + 'a>> {
    Box::pin(async move {
      client_message.add_frame(Frame::new_begin_frame().copy()).await;
      let n = list.len();
      for i in 0..n {
        let item = &mut list[i];
        if item.is_none() {
          client_message.add_frame(Frame::new_null_frame().copy()).await;
        } else {
          encoder(client_message, &mut item.as_mut().unwrap()).await;
        }
      }
      client_message.add_frame(Frame::new_end_frame().copy()).await;
    })
  }

  pub fn encode_nullable<'a, T: Send + Sync>(
    client_message: &'a mut ClientMessage,
    list: Option<&'a mut Vec<T>>,
    encoder: impl for<'b> Fn(&'b mut ClientMessage, &'b T) -> Pin<Box<dyn Future<Output=()> + Send + Sync + 'b>> + Send + Sync + 'a,
  ) -> Pin<Box<dyn Future<Output=()> + Send + Sync + 'a>> {
    Box::pin(async move {
      if list.is_none() {
        client_message.add_frame(Frame::new_null_frame().copy()).await;
      } else {
        Self::encode(client_message, list.unwrap(), encoder).await;
      }
    })
  }

  pub fn decode<'a, T: Send + Sync>(
    client_message: &'a mut ClientMessage,
    decoder: impl for<'b> Fn(&'b mut ClientMessage) -> Pin<Box<dyn Future<Output=T> + Send + Sync + 'b>> + Send + Sync + 'a,
  ) -> Pin<Box<dyn Future<Output=Vec<T>> + Send + Sync + 'a>> {
    Box::pin(async move {
      let mut result = vec![];
      client_message.next_frame().await;
      while !CodecUtil::next_frame_is_data_structure_end_frame(client_message).await {
        result.push(decoder(client_message).await);
      }
      client_message.next_frame().await;
      result
    })
  }

  pub fn decode_nullable<'a, T: Send + Sync>(
    client_message: &'a mut ClientMessage,
    decoder: impl for<'b> Fn(&'b mut ClientMessage) -> Pin<Box<dyn Future<Output=T> + Send + Sync + 'b>> + Send + Sync + 'a,
  ) -> Pin<Box<dyn Future<Output=Option<Vec<T>>> + Send + Sync + 'a>> {
    Box::pin(async move {
      if CodecUtil::next_frame_is_null_frame(client_message).await {
        None
      } else {
        Some(Self::decode(client_message, decoder).await)
      }
    })
  }

  pub fn decode_contains_nullable<'a, T>(
    client_message: &'a mut ClientMessage,
    decoder: impl for<'b> Fn(&'b mut ClientMessage) -> Pin<Box<dyn Future<Output=T> + 'b>> + 'a,
  ) -> Pin<Box<dyn Future<Output=Vec<Option<T>>> + 'a>> {
    Box::pin(async move {
      let mut result = vec![];
      client_message.next_frame().await;
      while !CodecUtil::next_frame_is_data_structure_end_frame(client_message).await {
        if CodecUtil::next_frame_is_null_frame(client_message).await {
          result.push(None);
        } else {
          result.push(Some(decoder(client_message).await));
        }
      }
      client_message.next_frame().await;
      result
    })
  }
}