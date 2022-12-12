use crate::codec_builtin::string_codec::StringCodec;
use crate::serialization::heap_data::HeapData;
use crate::codec_builtin::data_codec::DataCodec;
use crate::codec_builtin::codec_util::CodecUtil;

use std::mem::MaybeUninit;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::util::bits_util::BitsUtil;
use core::pin::Pin;
use std::future::Future;
use crate::codec_builtin::fix_sized_types_codec::FixSizedTypesCodec;



pub struct MapExecuteOnKeyCodec;

impl MapExecuteOnKeyCodec {

    // hex: 0x012E00
    const REQUEST_MESSAGE_TYPE: i32 = 77312;
    // hex: 0x012E01
    // RESPONSE_MESSAGE_TYPE = 77313

    const REQUEST_THREAD_ID_OFFSET: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;
    const REQUEST_INITIAL_FRAME_SIZE: usize = Self::REQUEST_THREAD_ID_OFFSET + BitsUtil::LONG_SIZE_IN_BYTES as usize;

    pub fn encode_request<'a>(name: &'a String, entry_processor: &'a HeapData, key: &'a HeapData, thread_id: &'a i64) -> Pin<Box<dyn Future<Output=ClientMessage> + Send + Sync + 'a>> {
        Box::pin(async move {
            let mut client_message = ClientMessage::create_for_encode().await;
            client_message.retryable = false;

            let initial_frame = Frame::create_initial_frame(Self::REQUEST_INITIAL_FRAME_SIZE, None);
            FixSizedTypesCodec::encode_long(&mut *initial_frame.content.lock().await, Self::REQUEST_THREAD_ID_OFFSET, thread_id).await;
            client_message.add_frame(initial_frame).await;
            client_message.set_message_type(Self::REQUEST_MESSAGE_TYPE).await;
            client_message.set_partition_id(-1).await;

            StringCodec::encode(&mut client_message, name).await;
            DataCodec::encode(&mut client_message, entry_processor).await;
            DataCodec::encode(&mut client_message, key).await;

            client_message
        })
    }


    pub fn decode_response<'a>(client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=Option<HeapData>> + Send + Sync + 'a>> {
        Box::pin(async move {
            // empty initial frame
            client_message.next_frame().await.unwrap();

            CodecUtil::decode_nullable(client_message, DataCodec::decode).await
        })
    }


}