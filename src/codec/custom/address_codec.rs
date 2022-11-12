use crate::connection::address::Address;
use crate::codec_builtin::string_codec::StringCodec;

use crate::codec_builtin::list_multi_frame_codec::ListMultiFrameCodec;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::codec_builtin::codec_util::CodecUtil;
use core::pin::Pin;
use std::future::Future;
use crate::codec_builtin::fix_sized_types_codec::FixSizedTypesCodec;
use crate::util::bits_util::BitsUtil;

pub struct AddressCodec;

impl AddressCodec {
    const PORT_OFFSET: usize = 0;
    const INITIAL_FRAME_SIZE: usize = Self::PORT_OFFSET + BitsUtil::INT_SIZE_IN_BYTES as usize;


    pub fn encode<'a>(client_message: &'a mut ClientMessage, address: &'a Address) -> Pin<Box<dyn Future<Output=()> + Send + Sync + 'a>> {
        Box::pin(async move {
            client_message.add_frame(Frame::new_begin_frame()).await;

            let mut initial_frame = Frame::create_initial_frame(Self::INITIAL_FRAME_SIZE, Some(ClientMessage::DEFAULT_FLAGS));
            FixSizedTypesCodec::encode_int(&mut *initial_frame.content.lock().await, Self::PORT_OFFSET, &address.port).await;
            client_message.add_frame(initial_frame).await;

            StringCodec::encode(client_message, &address.host).await;

            client_message.add_frame(Frame::new_end_frame()).await;
        })
    }

    pub fn decode<'a>(client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=Address> + Send + Sync + 'a>> {
        Box::pin(async move {
            client_message.next_frame().await.unwrap();
            let mut initial_frame = client_message.next_frame().await.unwrap();
            let port = FixSizedTypesCodec::decode_int(&mut *initial_frame.content.lock().await, Self::PORT_OFFSET).await;

            let host = StringCodec::decode(client_message).await;
            CodecUtil::fast_forward_to_end_frame(client_message).await;

            Address::new(host, port)
        })
    }
}