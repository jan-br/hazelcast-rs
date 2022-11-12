use uuid::Uuid;
use crate::codec_builtin::fix_sized_types_codec::FixSizedTypesCodec;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::util::bits_util::BitsUtil;

pub struct ListUUIDCodec;

impl ListUUIDCodec {
  pub async fn encode(client_message: &mut ClientMessage, items: &mut Vec<Uuid>) {
    let item_count = items.len();
    let mut frame = Frame::new_default_flags(vec![0; item_count * BitsUtil::UUID_SIZE_IN_BYTES as usize]);
    for i in 0..item_count {
      FixSizedTypesCodec::encode_uuid(&mut *frame.content.lock().await, i * BitsUtil::UUID_SIZE_IN_BYTES as usize, &items[i]).await;
    }
    client_message.add_frame(frame).await;
  }

  pub async fn decode(client_message: &mut ClientMessage) -> Vec<Uuid> {
    let mut frame = client_message.next_frame().await.unwrap();
    let item_count = frame.content.lock().await.len() / BitsUtil::UUID_SIZE_IN_BYTES as usize;
    let mut result = vec![Uuid::nil(); item_count];
    for i in 0..item_count {
      result[i] = FixSizedTypesCodec::decode_uuid(&mut *frame.content.lock().await, i * BitsUtil::UUID_SIZE_IN_BYTES as usize).await;
    }
    result
  }
}