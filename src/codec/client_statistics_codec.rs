use crate::codec_builtin::string_codec::StringCodec;
use crate::codec_builtin::byte_array_codec::ByteArrayCodec;

use std::mem::MaybeUninit;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::util::bits_util::BitsUtil;
use core::pin::Pin;
use std::future::Future;
use crate::codec_builtin::fix_sized_types_codec::FixSizedTypesCodec;



pub struct ClientStatisticsCodec;

impl ClientStatisticsCodec {

    // hex: 0x000C00
    const REQUEST_MESSAGE_TYPE: i32 = 3072;
    // hex: 0x000C01
    // RESPONSE_MESSAGE_TYPE = 3073

    const REQUEST_TIMESTAMP_OFFSET: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;
    const REQUEST_INITIAL_FRAME_SIZE: usize = Self::REQUEST_TIMESTAMP_OFFSET + BitsUtil::LONG_SIZE_IN_BYTES as usize;

    pub fn encode_request<'a>(timestamp: &'a i64, client_attributes: &'a String, metrics_blob: &'a Buffer) -> Pin<Box<dyn Future<Output=ClientMessage> + Send + Sync + 'a>> {
        Box::pin(async move {
            let mut client_message = ClientMessage::create_for_encode().await;
            client_message.retryable = false;

            let initial_frame = Frame::create_initial_frame(Self::REQUEST_INITIAL_FRAME_SIZE, None);
            FixSizedTypesCodec::encode_long(&mut *initial_frame.content.lock().await, Self::REQUEST_TIMESTAMP_OFFSET, timestamp).await;
            client_message.add_frame(initial_frame).await;
            client_message.set_message_type(Self::REQUEST_MESSAGE_TYPE).await;
            client_message.set_partition_id(-1).await;

            StringCodec::encode(&mut client_message, client_attributes).await;
            ByteArrayCodec::encode(&mut client_message, metrics_blob).await;

            client_message
        })
    }


}