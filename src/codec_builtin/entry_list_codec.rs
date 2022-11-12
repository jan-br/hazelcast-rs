use std::future::Future;
use std::pin::Pin;
use crate::codec_builtin::codec_util::CodecUtil;
use crate::protocol::client_message::{ClientMessage, Frame};

pub struct EntryListCodec;

impl EntryListCodec {
  pub fn encode<'a, K, V>(
    client_message: &'a mut ClientMessage,
    entries: &'a mut Vec<(K, V)>,
    key_encoder: impl for<'b> Fn(&'b mut ClientMessage, &'b mut K) -> Pin<Box<dyn Future<Output=()> + 'b>> + 'a,
    value_encoder: impl for<'b> Fn(&'b mut ClientMessage, &'b mut V) -> Pin<Box<dyn Future<Output=()> + 'b>> + 'a,
  ) -> Pin<Box<dyn Future<Output=()> + 'a>> {
    Box::pin(async move {
      client_message.add_frame(Frame::new_begin_frame().copy()).await;
      let n = entries.len();
      for i in 0..n {
        key_encoder(client_message, &mut entries[i].0).await;
        value_encoder(client_message, &mut entries[i].1).await;
      }
      client_message.add_frame(Frame::new_end_frame().copy()).await;
    })
  }

  pub fn encode_nullable<'a, K, V>(
    client_message: &'a mut ClientMessage,
    entries: Option<&'a mut Vec<(K, V)>>,
    key_encoder: impl for<'b> Fn(&'b mut ClientMessage, &'b mut K) -> Pin<Box<dyn Future<Output=()> + 'b>> + 'a,
    value_encoder: impl for<'b> Fn(&'b mut ClientMessage, &'b mut V) -> Pin<Box<dyn Future<Output=()> + 'b>> + 'a,
  ) -> Pin<Box<dyn Future<Output=()> + 'a>> {
    Box::pin(async move {
      if let Some(entries) = entries {
        Self::encode(client_message, entries, key_encoder, value_encoder).await;
      } else {
        client_message.add_frame(Frame::new_null_frame().copy()).await;
      }
    })
  }

  pub async fn decode<K, V>(
    client_message: &mut ClientMessage,
    key_decoder: impl Fn(&mut ClientMessage) -> K,
    value_decoder: impl Fn(&mut ClientMessage) -> V) -> Vec<(K, V)> {
    let mut result = vec![];
    client_message.next_frame().await;
    while !CodecUtil::next_frame_is_data_structure_end_frame(client_message).await {
      let key = key_decoder(client_message);
      let value = value_decoder(client_message);
      result.push((key, value));
    }
    client_message.next_frame().await;
    result
  }

  pub async fn decode_nullable<K, V>(client_message: &mut ClientMessage,
                                     key_decoder: impl Fn(&mut ClientMessage) -> K, value_decoder: impl Fn(&mut ClientMessage) -> V) -> Option<Vec<(K, V)>> {
    if CodecUtil::next_frame_is_null_frame(client_message).await {
      None
    } else {
      Some(Self::decode(client_message, key_decoder, value_decoder).await)
    }
  }
}