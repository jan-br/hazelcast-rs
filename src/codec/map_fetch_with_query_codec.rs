use crate::codec_builtin::string_codec::StringCodec;
use crate::codec_builtin::codec_builtin::EntryListIntegerIntegerCodec;
use crate::serialization::heap_data::HeapData;
use crate::codec_builtin::data_codec::DataCodec;
use crate::codec_builtin::list_multi_frame_codec::ListMultiFrameCodec;

use std::mem::MaybeUninit;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::util::bits_util::BitsUtil;
use core::pin::Pin;
use std::future::Future;
use crate::codec_builtin::fix_sized_types_codec::FixSizedTypesCodec;


/** @internal */
#[derive(Default, Clone)]
pub struct MapFetchWithQueryResponseParams {
    pub results: Data[],
    pub iteration_pointers: Vec<(i32, i32)>,
}


pub struct MapFetchWithQueryCodec;

impl MapFetchWithQueryCodec {

    // hex: 0x014000
    const REQUEST_MESSAGE_TYPE: i32 = 81920;
    // hex: 0x014001
    // RESPONSE_MESSAGE_TYPE = 81921

    const REQUEST_BATCH_OFFSET: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;
    const REQUEST_INITIAL_FRAME_SIZE: usize = Self::REQUEST_BATCH_OFFSET + BitsUtil::INT_SIZE_IN_BYTES as usize;

    pub fn encode_request<'a>(name: &'a String, iteration_pointers: &'a Vec<(i32, i32)>, batch: &'a i32, projection: &'a HeapData, predicate: &'a HeapData) -> Pin<Box<dyn Future<Output=ClientMessage> + Send + Sync + 'a>> {
        Box::pin(async move {
            let mut client_message = ClientMessage::create_for_encode().await;
            client_message.retryable = true;

            let initial_frame = Frame::create_initial_frame(Self::REQUEST_INITIAL_FRAME_SIZE, None);
            FixSizedTypesCodec::encode_int(&mut *initial_frame.content.lock().await, Self::REQUEST_BATCH_OFFSET, batch).await;
            client_message.add_frame(initial_frame).await;
            client_message.set_message_type(Self::REQUEST_MESSAGE_TYPE).await;
            client_message.set_partition_id(-1).await;

            StringCodec::encode(&mut client_message, name).await;
            EntryListIntegerIntegerCodec::encode(&mut client_message, iteration_pointers).await;
            DataCodec::encode(&mut client_message, projection).await;
            DataCodec::encode(&mut client_message, predicate).await;

            client_message
        })
    }

    pub fn decode_response<'a>(client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=MapFetchWithQueryResponseParams> + Send + Sync + 'a>> {
        Box::pin(async move {
            // empty initial frame
            client_message.next_frame().await.unwrap();
            #[allow(invalid_value)]
            let mut response = unsafe { MaybeUninit::<ClientAuthenticationResponseParams>::zeroed().assume_init() };

            response.results = ListMultiFrameCodec::decode_contains_nullable(client_message, DataCodec::decode).await;
            response.iteration_pointers = EntryListIntegerIntegerCodec::decode(client_message).await;
            response
        })
    }

}