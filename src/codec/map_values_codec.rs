use crate::codec_builtin::string_codec::StringCodec;
use crate::serialization::heap_data::HeapData;
use crate::codec_builtin::list_multi_frame_codec::ListMultiFrameCodec;
use crate::codec_builtin::data_codec::DataCodec;

use std::mem::MaybeUninit;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::util::bits_util::BitsUtil;
use core::pin::Pin;
use std::future::Future;



pub struct MapValuesCodec;

impl MapValuesCodec {

    // hex: 0x012400
    const REQUEST_MESSAGE_TYPE: i32 = 74752;
    // hex: 0x012401
    // RESPONSE_MESSAGE_TYPE = 74753

    const REQUEST_INITIAL_FRAME_SIZE: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;

    pub fn encode_request<'a>(name: &'a String) -> Pin<Box<dyn Future<Output=ClientMessage> + Send + Sync + 'a>> {
        Box::pin(async move {
            let mut client_message = ClientMessage::create_for_encode().await;
            client_message.retryable = true;

            let initial_frame = Frame::create_initial_frame(Self::REQUEST_INITIAL_FRAME_SIZE, None);
            client_message.add_frame(initial_frame).await;
            client_message.set_message_type(Self::REQUEST_MESSAGE_TYPE).await;
            client_message.set_partition_id(-1).await;

            StringCodec::encode(&mut client_message, name).await;

            client_message
        })
    }


    pub fn decode_response<'a>(client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=Vec<HeapData>> + Send + Sync + 'a>> {
        Box::pin(async move {
            // empty initial frame
            client_message.next_frame().await.unwrap();

            ListMultiFrameCodec::decode(client_message, DataCodec::decode).await
        })
    }


}