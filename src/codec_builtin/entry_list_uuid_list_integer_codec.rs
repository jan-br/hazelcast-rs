use std::future::Future;
use std::pin::Pin;
use uuid::Uuid;
use crate::codec_builtin::list_integer_codec::ListIntegerCodec;
use crate::codec_builtin::list_multi_frame_codec::ListMultiFrameCodec;
use crate::codec_builtin::list_uuid_codec::ListUUIDCodec;
use crate::protocol::client_message::{ClientMessage, Frame};

pub struct EntryListUUIDListIntegerCodec;

impl EntryListUUIDListIntegerCodec {
  pub fn encode<'a>(client_message: &'a mut ClientMessage, entries: &'a mut Vec<(Uuid, Vec<i32>)>) -> Pin<Box<dyn Future<Output=()> + 'a>> {
    Box::pin(async move {
      let entry_count = entries.len();
      let mut keys = vec![Uuid::nil(); entry_count];
      client_message.add_frame(Frame::new_begin_frame().copy()).await;
      for i in 0..entry_count {
        keys[i] = entries[i].0;
        ListIntegerCodec::encode(client_message, &mut entries[i].1).await;
      }
      client_message.add_frame(Frame::new_end_frame().copy()).await;
      ListUUIDCodec::encode(client_message, &mut keys).await;
    })
  }

  pub fn decode<'a>(
    client_message: &'a mut ClientMessage
  ) -> Pin<Box<dyn Future<Output=Vec<(Uuid, Vec<i32>)>> + Send + Sync + 'a>> {
    Box::pin(async move {
      let values = ListMultiFrameCodec::decode(client_message, ListIntegerCodec::decode).await;
      let keys = ListUUIDCodec::decode(client_message).await;

      let mut result = vec![(Uuid::nil(), vec![]); keys.len()];
      for i in 0..result.len() {
        result[i] = (keys[i], values[i].clone());
      }
      result
    })
  }
}