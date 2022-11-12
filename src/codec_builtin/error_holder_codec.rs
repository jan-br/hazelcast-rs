use std::future::Future;
use std::pin::Pin;
use futures::future::err;
use crate::codec_builtin::codec_util::CodecUtil;
use crate::codec_builtin::fix_sized_types_codec::FixSizedTypesCodec;
use crate::codec_builtin::list_multi_frame_codec::ListMultiFrameCodec;
use crate::codec_builtin::stack_trace_element_codec::StackTraceElementCodec;
use crate::codec_builtin::string_codec::StringCodec;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::protocol::error_holder::ErrorHolder;
use crate::util::bits_util::BitsUtil;

pub struct ErrorHolderCodec;

impl ErrorHolderCodec {
  pub const ERROR_CODE_OFFSET: i32 = 0;
  pub const INITIAL_FRAME_SIZE: i32 = Self::ERROR_CODE_OFFSET + BitsUtil::INT_SIZE_IN_BYTES;

  pub fn encode<'a>(client_message: &'a mut ClientMessage, error_holder: &'a ErrorHolder) -> Pin<Box<dyn Future<Output=()> + 'a>> {
    Box::pin(async move {
      client_message.add_frame(Frame::new_begin_frame()).await;

      let initial_frame = Frame::create_initial_frame(Self::INITIAL_FRAME_SIZE as usize, Some(ClientMessage::DEFAULT_FLAGS));
      FixSizedTypesCodec::encode_int(&mut *initial_frame.content.lock().await, Self::ERROR_CODE_OFFSET as usize, &error_holder.error_code).await;
      client_message.add_frame(initial_frame).await;

      StringCodec::encode(client_message, &error_holder.class_name).await;
      CodecUtil::encode_nullable(client_message, &error_holder.message, StringCodec::encode).await;
      ListMultiFrameCodec::encode(client_message, &error_holder.stack_trace_elements, StackTraceElementCodec::encode).await;
      client_message.add_frame(Frame::new_end_frame()).await;
    })
  }

  pub fn decode<'a>(client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=ErrorHolder> + Send + Sync + 'a>> {
    Box::pin(async move {
      client_message.next_frame().await;

      let initial_frame = client_message.next_frame().await.unwrap();
      let error_code = FixSizedTypesCodec::decode_int(&mut *initial_frame.content.lock().await, Self::ERROR_CODE_OFFSET as usize).await;

      let class_name = StringCodec::decode(client_message).await;
      let message = CodecUtil::decode_nullable(client_message, StringCodec::decode).await;
      let stack_trace_elements = ListMultiFrameCodec::decode(client_message, StackTraceElementCodec::decode).await;

      CodecUtil::fast_forward_to_end_frame(client_message).await;

      ErrorHolder {
        error_code,
        class_name,
        message,
        stack_trace_elements,
      }
    })
  }
}