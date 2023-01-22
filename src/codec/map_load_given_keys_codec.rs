use crate::codec_builtin::string_codec::StringCodec;
use crate::serialization::heap_data::HeapData;
use crate::codec_builtin::list_multi_frame_codec::ListMultiFrameCodec;
use crate::codec_builtin::data_codec::DataCodec;

use std::mem::MaybeUninit;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::util::bits_util::BitsUtil;
use core::pin::Pin;
use std::future::Future;
use crate::codec_builtin::fix_sized_types_codec::FixSizedTypesCodec;



pub struct MapLoadGivenKeysCodec;

impl MapLoadGivenKeysCodec {

    // hex: 0x012100
    const REQUEST_MESSAGE_TYPE: i32 = 73984;
    // hex: 0x012101
    // RESPONSE_MESSAGE_TYPE = 73985

    const REQUEST_REPLACE_EXISTING_VALUES_OFFSET: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;
    const REQUEST_INITIAL_FRAME_SIZE: usize = Self::REQUEST_REPLACE_EXISTING_VALUES_OFFSET + BitsUtil::BOOLEAN_SIZE_IN_BYTES as usize;

    pub fn encode_request<'a>(name: &'a String, keys: &'a Vec<HeapData>, replace_existing_values: &'a bool) -> Pin<Box<dyn Future<Output=ClientMessage> + Send + Sync + 'a>> {
        Box::pin(async move {
            let mut client_message = ClientMessage::create_for_encode().await;
            client_message.retryable = false;

            let initial_frame = Frame::create_initial_frame(Self::REQUEST_INITIAL_FRAME_SIZE, None);
            FixSizedTypesCodec::encode_boolean(&mut *initial_frame.content.lock().await, Self::REQUEST_REPLACE_EXISTING_VALUES_OFFSET, replace_existing_values).await;
            client_message.add_frame(initial_frame).await;
            client_message.set_message_type(Self::REQUEST_MESSAGE_TYPE).await;
            client_message.set_partition_id(-1).await;

            StringCodec::encode(&mut client_message, name).await;
            ListMultiFrameCodec::encode(&mut client_message, keys, DataCodec::encode).await;

            client_message
        })
    }


}