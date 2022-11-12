use std::pin::Pin;

use futures::Future;

use crate::codec_builtin::fix_sized_types_codec::FixSizedTypesCodec;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::util::bits_util::BitsUtil;

pub struct ListIntegerCodec;

impl ListIntegerCodec {
  pub async fn encode(client_message: &mut ClientMessage, list: &mut Vec<i32>) {
    let item_count = list.len();
    let mut frame = Frame::new_default_flags(vec![0; item_count * BitsUtil::INT_SIZE_IN_BYTES as usize]);
    for i in 0..item_count {
      FixSizedTypesCodec::encode_int(&mut *frame.content.lock().await, i * BitsUtil::INT_SIZE_IN_BYTES as usize, &list[i]).await;
    }
    client_message.add_frame(frame).await;
  }

  pub fn decode<'a>(client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=Vec<i32>> + Send + Sync + 'a>> {
    Box::pin(async move {
      let frame = client_message.next_frame().await.unwrap();
      let item_count = frame.content.lock().await.len() / BitsUtil::INT_SIZE_IN_BYTES as usize;
      let mut result = vec![0; item_count];
      for i in 0..item_count {
        result[i] = FixSizedTypesCodec::decode_int(&*frame.content.lock().await, i * BitsUtil::INT_SIZE_IN_BYTES as usize).await;
      }
      result
    })
  }
}