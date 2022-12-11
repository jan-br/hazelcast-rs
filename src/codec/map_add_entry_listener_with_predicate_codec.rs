use crate::codec_builtin::string_codec::StringCodec;
use crate::serialization::heap_data::HeapData;
use crate::codec_builtin::data_codec::DataCodec;
use uuid::Uuid;
use crate::codec_builtin::codec_util::CodecUtil;

use std::mem::MaybeUninit;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::util::bits_util::BitsUtil;
use core::pin::Pin;
use std::future::Future;
use crate::codec_builtin::fix_sized_types_codec::FixSizedTypesCodec;



pub struct MapAddEntryListenerWithPredicateCodec;

impl MapAddEntryListenerWithPredicateCodec {

    // hex: 0x011700
    const REQUEST_MESSAGE_TYPE: i32 = 71424;
    // hex: 0x011701
    // RESPONSE_MESSAGE_TYPE = 71425
    // hex: 0x011702
    const EVENT_ENTRY_MESSAGE_TYPE: i32 = 71426;

    const REQUEST_INCLUDE_VALUE_OFFSET: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;
    const REQUEST_LISTENER_FLAGS_OFFSET: usize = Self::REQUEST_INCLUDE_VALUE_OFFSET + BitsUtil::BOOLEAN_SIZE_IN_BYTES as usize;
    const REQUEST_LOCAL_ONLY_OFFSET: usize = Self::REQUEST_LISTENER_FLAGS_OFFSET + BitsUtil::INT_SIZE_IN_BYTES as usize;
    const REQUEST_INITIAL_FRAME_SIZE: usize = Self::REQUEST_LOCAL_ONLY_OFFSET + BitsUtil::BOOLEAN_SIZE_IN_BYTES as usize;
    const RESPONSE_RESPONSE_OFFSET: usize = ClientMessage::RESPONSE_BACKUP_ACKS_OFFSET as usize + BitsUtil::BYTE_SIZE_IN_BYTES as usize;
    const EVENT_ENTRY_EVENT_TYPE_OFFSET: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;
    const EVENT_ENTRY_UUID_OFFSET: usize = EVENT_ENTRY_EVENT_TYPE_OFFSET as usize+ BitsUtil::INT_SIZE_IN_BYTES as usize;
    const EVENT_ENTRY_NUMBER_OF_AFFECTED_ENTRIES_OFFSET: usize = EVENT_ENTRY_UUID_OFFSET as usize+ BitsUtil::UUID_SIZE_IN_BYTES as usize;

    pub fn encode_request<'a>(name: &'a String, predicate: &'a HeapData, include_value: &'a bool, listener_flags: &'a i32, local_only: &'a bool) -> Pin<Box<dyn Future<Output=ClientMessage> + Send + Sync + 'a>> {
        Box::pin(async move {
            let mut client_message = ClientMessage::create_for_encode().await;
            client_message.retryable = false;

            let initial_frame = Frame::create_initial_frame(Self::REQUEST_INITIAL_FRAME_SIZE, None);
            FixSizedTypesCodec::encode_boolean(&mut *initial_frame.content.lock().await, Self::REQUEST_INCLUDE_VALUE_OFFSET, include_value).await;
            FixSizedTypesCodec::encode_int(&mut *initial_frame.content.lock().await, Self::REQUEST_LISTENER_FLAGS_OFFSET, listener_flags).await;
            FixSizedTypesCodec::encode_boolean(&mut *initial_frame.content.lock().await, Self::REQUEST_LOCAL_ONLY_OFFSET, local_only).await;
            client_message.add_frame(initial_frame).await;
            client_message.set_message_type(Self::REQUEST_MESSAGE_TYPE).await;
            client_message.set_partition_id(-1).await;

            StringCodec::encode(&mut client_message, name).await;
            DataCodec::encode(&mut client_message, predicate).await;

            client_message
        })
    }


    pub fn decode_response<'a>(client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=Uuid> + Send + Sync + 'a>> {
        Box::pin(async move {
            let initial_frame = client_message.next_frame().await.unwrap();

            FixSizedTypesCodec::decode_uuid(&*initial_frame.content.lock().await, Self::RESPONSE_RESPONSE_OFFSET).await
        })
    }


    pub async fn handle(client_message: &mut ClientMessage, handle_entry_event: Option<impl Fn(HeapData, HeapData, HeapData, HeapData, i32, Uuid, i32)>) {
        let message_type = client_message.get_message_type().await;
        if message_type == Self::EVENT_ENTRY_MESSAGE_TYPE && handle_entry_event.is_some() {
            let initial_frame = client_message.next_frame().await.unwrap();
            let event_type = FixSizedTypesCodec::decode_int(&mut *initial_frame.content.lock().await, Self::EVENT_ENTRY_EVENT_TYPE_OFFSET).await;
            let uuid = FixSizedTypesCodec::decode_uuid(&mut *initial_frame.content.lock().await, Self::EVENT_ENTRY_UUID_OFFSET).await;
            let number_of_affected_entries = FixSizedTypesCodec::decode_int(&mut *initial_frame.content.lock().await, Self::EVENT_ENTRY_NUMBER_OF_AFFECTED_ENTRIES_OFFSET).await;
            let key = CodecUtil::decode_nullable(client_message, DataCodec::decode).await;
            let value = CodecUtil::decode_nullable(client_message, DataCodec::decode).await;
            let old_value = CodecUtil::decode_nullable(client_message, DataCodec::decode).await;
            let merging_value = CodecUtil::decode_nullable(client_message, DataCodec::decode).await;
            handle_entry_event.unwrap()(key, value, old_value, merging_value, event_type, uuid, number_of_affected_entries);
            return;
        }
    }
}