use std::collections::HashMap;
use std::future::Future;
use std::hash::Hash;
use std::pin::Pin;
use crate::codec_builtin::codec_util::CodecUtil;
use crate::protocol::client_message::{ClientMessage, Frame};

pub struct MapCodec {}

impl MapCodec {
  pub fn encode<'a, K: Send + Sync, V: Send + Sync>(
    client_message: &'a mut ClientMessage,
    map: &'a HashMap<K, V>,
    key_encoder: impl for<'b> Fn(&'b mut ClientMessage, &'b K) -> Pin<Box<dyn Future<Output=()> + Send + Sync + 'b>> + Send + Sync + 'a,
    value_encoder: impl for<'b> Fn(&'b mut ClientMessage, &'b V) -> Pin<Box<dyn Future<Output=()> + Send + Sync + 'b>> + Send + Sync + 'a,
  ) -> Pin<Box<dyn Future<Output=()> + Send + Sync + 'a>> {
    Box::pin(async move {
      client_message.add_frame(Frame::new_begin_frame().copy()).await;
      for (mut key, mut value) in map {
        key_encoder(client_message, &key).await;
        value_encoder(client_message, &value).await;
      }
      client_message.add_frame(Frame::new_end_frame().copy()).await;
    })
  }

  pub fn encode_nullable<'a, K: Send + Sync, V: Send + Sync>(
    client_message: &'a mut ClientMessage,
    map: Option<&'a HashMap<K, V>>,
    key_encoder: impl for<'b> Fn(&'b mut ClientMessage, &'b K) -> Pin<Box<dyn Future<Output=()> + Send + Sync + 'b>> + Send + Sync + 'a,
    value_encoder: impl for<'b> Fn(&'b mut ClientMessage, &'b V) -> Pin<Box<dyn Future<Output=()> + Send + Sync + 'b>> + Send + Sync + 'a,
  ) -> Pin<Box<dyn Future<Output=()> + Send + Sync + 'a>> {
    Box::pin(async move {
      if map.is_none() {
        client_message.add_frame(Frame::new_null_frame().copy()).await;
      } else {
        MapCodec::encode(client_message, map.unwrap(), key_encoder, value_encoder).await;
      }
    })
  }

  pub fn decode<'a, K: Send + Sync + Eq + Hash, V: Send + Sync>(
    client_message: &'a mut ClientMessage,
    key_decoder: impl for<'b> Fn(&'b mut ClientMessage) -> Pin<Box<dyn Future<Output=K> + Send + Sync + 'b>> + Send + Sync + 'a,
    value_decoder: impl for<'b> Fn(&'b mut ClientMessage) -> Pin<Box<dyn Future<Output=V> + Send + Sync + 'b>> + Send + Sync + 'a,
  ) -> Pin<Box<dyn Future<Output=HashMap<K, V>> + Send + Sync + 'a>> {
    Box::pin(async move {
      let mut result = HashMap::new();
      client_message.next_frame().await;

      while !CodecUtil::next_frame_is_data_structure_end_frame(client_message).await {
        let key = key_decoder(client_message).await;
        let value = value_decoder(client_message).await;
        result.insert(key, value);
      }
      client_message.next_frame().await;
      result
    })
  }

  pub fn decode_nullable<'a, K: Send + Sync + Eq + Hash, V: Send + Sync>(
    client_message: &'a mut ClientMessage,
    key_decoder: impl for<'b> Fn(&'b mut ClientMessage) -> Pin<Box<dyn Future<Output=K> + Send + Sync + 'b>> + Send + Sync + 'a,
    value_decoder: impl for<'b> Fn(&'b mut ClientMessage) -> Pin<Box<dyn Future<Output=V> + Send + Sync + 'b>> + Send + Sync + 'a,
  ) -> Pin<Box<dyn Future<Output=Option<HashMap<K, V>>> + Send + Sync + 'a>> {
    Box::pin(async move {
      if CodecUtil::next_frame_is_null_frame(client_message).await {
        None
      } else {
        Some(MapCodec::decode(client_message, key_decoder, value_decoder).await)
      }
    })
  }
}