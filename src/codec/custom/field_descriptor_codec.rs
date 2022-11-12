use crate::serialization::generic_record::field_descriptor::FieldDescriptor;
use crate::codec_builtin::string_codec::StringCodec;

use crate::codec_builtin::list_multi_frame_codec::ListMultiFrameCodec;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::codec_builtin::codec_util::CodecUtil;
use core::pin::Pin;
use std::future::Future;
use crate::codec_builtin::fix_sized_types_codec::FixSizedTypesCodec;
use crate::util::bits_util::BitsUtil;

pub struct FieldDescriptorCodec;

impl FieldDescriptorCodec {
    const KIND_OFFSET: usize = 0;
    const INITIAL_FRAME_SIZE: usize = Self::KIND_OFFSET + BitsUtil::INT_SIZE_IN_BYTES as usize;


    pub fn encode<'a>(client_message: &'a mut ClientMessage, field_descriptor: &'a FieldDescriptor) -> Pin<Box<dyn Future<Output=()> + Send + Sync + 'a>> {
        Box::pin(async move {
            client_message.add_frame(Frame::new_begin_frame()).await;

            let mut initial_frame = Frame::create_initial_frame(Self::INITIAL_FRAME_SIZE, Some(ClientMessage::DEFAULT_FLAGS));
            FixSizedTypesCodec::encode_int(&mut *initial_frame.content.lock().await, Self::KIND_OFFSET, &field_descriptor.kind).await;
            client_message.add_frame(initial_frame).await;

            StringCodec::encode(client_message, &field_descriptor.field_name).await;

            client_message.add_frame(Frame::new_end_frame()).await;
        })
    }

    pub fn decode<'a>(client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=FieldDescriptor> + Send + Sync + 'a>> {
        Box::pin(async move {
            client_message.next_frame().await.unwrap();
            let mut initial_frame = client_message.next_frame().await.unwrap();
            let kind = FixSizedTypesCodec::decode_int(&mut *initial_frame.content.lock().await, Self::KIND_OFFSET).await;

            let field_name = StringCodec::decode(client_message).await;
            CodecUtil::fast_forward_to_end_frame(client_message).await;

            FieldDescriptor::new(field_name, kind)
        })
    }
}