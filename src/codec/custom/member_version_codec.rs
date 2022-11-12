use crate::core::member::version::MemberVersion;

use crate::protocol::client_message::{ClientMessage, Frame};
use crate::codec_builtin::codec_util::CodecUtil;
use core::pin::Pin;
use std::future::Future;
use crate::codec_builtin::fix_sized_types_codec::FixSizedTypesCodec;
use crate::util::bits_util::BitsUtil;

pub struct MemberVersionCodec;

impl MemberVersionCodec {
    const MAJOR_OFFSET: usize = 0;
    const MINOR_OFFSET: usize = Self::MAJOR_OFFSET + BitsUtil::BYTE_SIZE_IN_BYTES as usize;
    const PATCH_OFFSET: usize = Self::MINOR_OFFSET + BitsUtil::BYTE_SIZE_IN_BYTES as usize;
    const INITIAL_FRAME_SIZE: usize = Self::PATCH_OFFSET + BitsUtil::BYTE_SIZE_IN_BYTES as usize;


    pub fn encode<'a>(client_message: &'a mut ClientMessage, member_version: &'a MemberVersion) -> Pin<Box<dyn Future<Output=()> + Send + Sync + 'a>> {
        Box::pin(async move {
            client_message.add_frame(Frame::new_begin_frame()).await;

            let mut initial_frame = Frame::create_initial_frame(Self::INITIAL_FRAME_SIZE, Some(ClientMessage::DEFAULT_FLAGS));
            FixSizedTypesCodec::encode_byte(&mut *initial_frame.content.lock().await, Self::MAJOR_OFFSET, &member_version.major).await;
            FixSizedTypesCodec::encode_byte(&mut *initial_frame.content.lock().await, Self::MINOR_OFFSET, &member_version.minor).await;
            FixSizedTypesCodec::encode_byte(&mut *initial_frame.content.lock().await, Self::PATCH_OFFSET, &member_version.patch).await;
            client_message.add_frame(initial_frame).await;

            client_message.add_frame(Frame::new_end_frame()).await;
        })
    }

    pub fn decode<'a>(client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=MemberVersion> + Send + Sync + 'a>> {
        Box::pin(async move {
            client_message.next_frame().await.unwrap();
            let mut initial_frame = client_message.next_frame().await.unwrap();
            let major = FixSizedTypesCodec::decode_byte(&mut *initial_frame.content.lock().await, Self::MAJOR_OFFSET).await;
            let minor = FixSizedTypesCodec::decode_byte(&mut *initial_frame.content.lock().await, Self::MINOR_OFFSET).await;
            let patch = FixSizedTypesCodec::decode_byte(&mut *initial_frame.content.lock().await, Self::PATCH_OFFSET).await;
            CodecUtil::fast_forward_to_end_frame(client_message).await;

            MemberVersion::new(major, minor, patch)
        })
    }
}