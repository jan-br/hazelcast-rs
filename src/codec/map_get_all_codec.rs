use crate::codec_builtin::string_codec::StringCodec;
use crate::serialization::heap_data::HeapData;
use crate::codec_builtin::list_multi_frame_codec::ListMultiFrameCodec;
use crate::codec_builtin::data_codec::DataCodec;
use crate::codec_builtin::entry_list_codec::EntryListCodec;

use std::mem::MaybeUninit;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::util::bits_util::BitsUtil;
use core::pin::Pin;
use std::future::Future;



pub struct MapGetAllCodec;

impl MapGetAllCodec {

    // hex: 0x012300
    const REQUEST_MESSAGE_TYPE: i32 = 74496;
    // hex: 0x012301
    // RESPONSE_MESSAGE_TYPE = 74497

    const REQUEST_INITIAL_FRAME_SIZE: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;

    pub fn encode_request<'a>(name: &'a String, keys: &'a Vec<HeapData>) -> Pin<Box<dyn Future<Output=ClientMessage> + Send + Sync + 'a>> {
        Box::pin(async move {
            let mut client_message = ClientMessage::create_for_encode().await;
            client_message.retryable = false;

            let initial_frame = Frame::create_initial_frame(Self::REQUEST_INITIAL_FRAME_SIZE, None);
            client_message.add_frame(initial_frame).await;
            client_message.set_message_type(Self::REQUEST_MESSAGE_TYPE).await;
            client_message.set_partition_id(-1).await;

            StringCodec::encode(&mut client_message, name).await;
            ListMultiFrameCodec::encode(&mut client_message, keys, DataCodec::encode).await;

            client_message
        })
    }


    pub fn decode_response<'a>(client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=Array<[Data, Data]>> + Send + Sync + 'a>> {
        Box::pin(async move {
            // empty initial frame
            client_message.next_frame().await.unwrap();

            EntryListCodec::decode(client_message, DataCodec::decode, DataCodec::decode).await
        })
    }


}