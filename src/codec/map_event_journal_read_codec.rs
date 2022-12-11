use crate::codec_builtin::string_codec::StringCodec;
use crate::serialization::heap_data::HeapData;
use crate::codec_builtin::data_codec::DataCodec;
use crate::codec_builtin::codec_util::CodecUtil;
use crate::codec_builtin::list_multi_frame_codec::ListMultiFrameCodec;
use crate::codec_builtin::long_array_codec::LongArrayCodec;

use std::mem::MaybeUninit;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::util::bits_util::BitsUtil;
use core::pin::Pin;
use std::future::Future;
use crate::codec_builtin::fix_sized_types_codec::FixSizedTypesCodec;


/** @internal */
#[derive(Default, Clone)]
pub struct MapEventJournalReadResponseParams {
    pub read_count: i32,
    pub items: Data[],
    pub item_seqs: Option<Long[]>,
    pub next_seq: i64,
}


pub struct MapEventJournalReadCodec;

impl MapEventJournalReadCodec {

    // hex: 0x014200
    const REQUEST_MESSAGE_TYPE: i32 = 82432;
    // hex: 0x014201
    // RESPONSE_MESSAGE_TYPE = 82433

    const REQUEST_START_SEQUENCE_OFFSET: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;
    const REQUEST_MIN_SIZE_OFFSET: usize = Self::REQUEST_START_SEQUENCE_OFFSET + BitsUtil::LONG_SIZE_IN_BYTES as usize;
    const REQUEST_MAX_SIZE_OFFSET: usize = Self::REQUEST_MIN_SIZE_OFFSET + BitsUtil::INT_SIZE_IN_BYTES as usize;
    const REQUEST_INITIAL_FRAME_SIZE: usize = Self::REQUEST_MAX_SIZE_OFFSET + BitsUtil::INT_SIZE_IN_BYTES as usize;
    const RESPONSE_READ_COUNT_OFFSET: usize = ClientMessage::RESPONSE_BACKUP_ACKS_OFFSET as usize + BitsUtil::BYTE_SIZE_IN_BYTES as usize;
    const RESPONSE_NEXT_SEQ_OFFSET: usize = Self::RESPONSE_READ_COUNT_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;

    pub fn encode_request<'a>(name: &'a String, start_sequence: &'a i64, min_size: &'a i32, max_size: &'a i32, predicate: &'a Option<&'a HeapData>, projection: &'a Option<&'a HeapData>) -> Pin<Box<dyn Future<Output=ClientMessage> + Send + Sync + 'a>> {
        Box::pin(async move {
            let mut client_message = ClientMessage::create_for_encode().await;
            client_message.retryable = true;

            let initial_frame = Frame::create_initial_frame(Self::REQUEST_INITIAL_FRAME_SIZE, None);
            FixSizedTypesCodec::encode_long(&mut *initial_frame.content.lock().await, Self::REQUEST_START_SEQUENCE_OFFSET, start_sequence).await;
            FixSizedTypesCodec::encode_int(&mut *initial_frame.content.lock().await, Self::REQUEST_MIN_SIZE_OFFSET, min_size).await;
            FixSizedTypesCodec::encode_int(&mut *initial_frame.content.lock().await, Self::REQUEST_MAX_SIZE_OFFSET, max_size).await;
            client_message.add_frame(initial_frame).await;
            client_message.set_message_type(Self::REQUEST_MESSAGE_TYPE).await;
            client_message.set_partition_id(-1).await;

            StringCodec::encode(&mut client_message, name).await;
            CodecUtil::encode_nullable(&mut client_message, predicate, DataCodec::encode).await;
            CodecUtil::encode_nullable(&mut client_message, projection, DataCodec::encode).await;

            client_message
        })
    }

    pub fn decode_response<'a>(client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=MapEventJournalReadResponseParams> + Send + Sync + 'a>> {
        Box::pin(async move {
            let initial_frame = client_message.next_frame().await.unwrap();
            #[allow(invalid_value)]
            let mut response = unsafe { MaybeUninit::<ClientAuthenticationResponseParams>::zeroed().assume_init() };

            response.read_count = FixSizedTypesCodec::decode_int(&*initial_frame.content.lock().await, Self::RESPONSE_READ_COUNT_OFFSET).await;
            response.next_seq = FixSizedTypesCodec::decode_long(&*initial_frame.content.lock().await, Self::RESPONSE_NEXT_SEQ_OFFSET).await;
            response.items = ListMultiFrameCodec::decode(client_message, DataCodec::decode).await;
            response.item_seqs = CodecUtil::decode_nullable(client_message, LongArrayCodec::decode).await;
            response
        })
    }

}