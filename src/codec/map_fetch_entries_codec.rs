use crate::codec_builtin::string_codec::StringCodec;
use crate::codec_builtin::codec_builtin::EntryListIntegerIntegerCodec;
use crate::codec_builtin::entry_list_codec::EntryListCodec;
use crate::codec_builtin::data_codec::DataCodec;
use crate::serialization::heap_data::HeapData;

use std::mem::MaybeUninit;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::util::bits_util::BitsUtil;
use core::pin::Pin;
use std::future::Future;
use crate::codec_builtin::fix_sized_types_codec::FixSizedTypesCodec;


/** @internal */
#[derive(Default, Clone)]
pub struct MapFetchEntriesResponseParams {
    pub iteration_pointers: Vec<(i32, i32)>,
    pub entries: Array<[Data, Data]>,
}


pub struct MapFetchEntriesCodec;

impl MapFetchEntriesCodec {

    // hex: 0x013800
    const REQUEST_MESSAGE_TYPE: i32 = 79872;
    // hex: 0x013801
    // RESPONSE_MESSAGE_TYPE = 79873

    const REQUEST_BATCH_OFFSET: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;
    const REQUEST_INITIAL_FRAME_SIZE: usize = Self::REQUEST_BATCH_OFFSET + BitsUtil::INT_SIZE_IN_BYTES as usize;

    pub fn encode_request<'a>(name: &'a String, iteration_pointers: &'a Vec<(i32, i32)>, batch: &'a i32) -> Pin<Box<dyn Future<Output=ClientMessage> + Send + Sync + 'a>> {
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

            client_message
        })
    }

    pub fn decode_response<'a>(client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=MapFetchEntriesResponseParams> + Send + Sync + 'a>> {
        Box::pin(async move {
            // empty initial frame
            client_message.next_frame().await.unwrap();
            #[allow(invalid_value)]
            let mut response = unsafe { MaybeUninit::<ClientAuthenticationResponseParams>::zeroed().assume_init() };

            response.iteration_pointers = EntryListIntegerIntegerCodec::decode(client_message).await;
            response.entries = EntryListCodec::decode(client_message, DataCodec::decode, DataCodec::decode).await;
            response
        })
    }

}