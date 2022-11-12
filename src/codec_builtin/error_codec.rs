use std::future::Future;
use std::pin::Pin;
use crate::codec_builtin::error_holder_codec::ErrorHolderCodec;
use crate::codec_builtin::list_multi_frame_codec::ListMultiFrameCodec;
use crate::protocol::client_message::ClientMessage;
use crate::protocol::error_holder::ErrorHolder;

pub struct ErrorCodec;

impl ErrorCodec {
  pub const EXCEPTION_MESSAGE_TYPE: i32 = 0;

  pub fn decode<'a>(client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=Vec<ErrorHolder>> + Send + Sync + 'a>> {
    Box::pin(async move {
      client_message.next_frame().await;
      ListMultiFrameCodec::decode(client_message, ErrorHolderCodec::decode).await;
      vec![]
    })
  }
}