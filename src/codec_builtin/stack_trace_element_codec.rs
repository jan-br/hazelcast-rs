use std::future::Future;
use std::pin::Pin;
use crate::codec_builtin::codec_util::CodecUtil;
use crate::codec_builtin::fix_sized_types_codec::FixSizedTypesCodec;
use crate::codec_builtin::string_codec::StringCodec;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::protocol::stack_trace_element::StackTraceElement;
use crate::util::bits_util::BitsUtil;

pub struct StackTraceElementCodec;

impl StackTraceElementCodec {
  pub const LINE_NUMBER_OFFSET: i32 = 0;
  pub const INITIAL_FRAME_SIZE: i32 = Self::LINE_NUMBER_OFFSET + BitsUtil::INT_SIZE_IN_BYTES;

  pub fn encode<'a>(client_message: &'a mut ClientMessage, stack_trace_element: &'a StackTraceElement) -> Pin<Box<dyn Future<Output=()> + Send + Sync + 'a>> {
    Box::pin(async move {
      client_message.add_frame(Frame::new_begin_frame()).await;

      let initial_frame = Frame::create_initial_frame(Self::INITIAL_FRAME_SIZE as usize, Some(ClientMessage::DEFAULT_FLAGS));
      FixSizedTypesCodec::encode_int(&mut *initial_frame.content.lock().await, Self::LINE_NUMBER_OFFSET as usize, &stack_trace_element.line_number).await;
      client_message.add_frame(initial_frame).await;

      StringCodec::encode(client_message, &stack_trace_element.class_name).await;
      StringCodec::encode(client_message, &stack_trace_element.method_name).await;
      CodecUtil::encode_nullable(client_message, &stack_trace_element.file_name, StringCodec::encode).await;

      client_message.add_frame(Frame::new_end_frame()).await;
    })
  }

  pub fn decode<'a>(client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=StackTraceElement> + Send + Sync + 'a>> {
    Box::pin(async move {
      client_message.next_frame().await;
      let initial_frame = client_message.next_frame().await.unwrap();
      let line_number = FixSizedTypesCodec::decode_int(&mut *initial_frame.content.lock().await, Self::LINE_NUMBER_OFFSET as usize).await;
      let class_name = StringCodec::decode(client_message).await;
      let method_name = StringCodec::decode(client_message).await;
      let file_name = CodecUtil::decode_nullable(client_message, StringCodec::decode).await;

      CodecUtil::fast_forward_to_end_frame(client_message).await;
      StackTraceElement {
        line_number,
        class_name,
        method_name,
        file_name,
      }
    })
  }
}