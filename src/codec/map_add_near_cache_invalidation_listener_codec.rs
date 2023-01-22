use crate::codec_builtin::string_codec::StringCodec;
use uuid::Uuid;
use crate::serialization::heap_data::HeapData;
use crate::codec_builtin::data_codec::DataCodec;
use crate::codec_builtin::codec_util::CodecUtil;
use crate::codec_builtin::list_multi_frame_codec::ListMultiFrameCodec;
use crate::codec_builtin::codec_builtin::ListUUIDCodec;
use crate::codec_builtin::codec_builtin::ListLongCodec;

use std::mem::MaybeUninit;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::util::bits_util::BitsUtil;
use core::pin::Pin;
use std::future::Future;
use crate::codec_builtin::fix_sized_types_codec::FixSizedTypesCodec;



pub struct MapAddNearCacheInvalidationListenerCodec;

impl MapAddNearCacheInvalidationListenerCodec {

    // hex: 0x013F00
    const REQUEST_MESSAGE_TYPE: i32 = 81664;
    // hex: 0x013F01
    // RESPONSE_MESSAGE_TYPE = 81665
    // hex: 0x013F02
    const EVENT_I_MAP_INVALIDATION_MESSAGE_TYPE: i32 = 81666;
    // hex: 0x013F03
    const EVENT_I_MAP_BATCH_INVALIDATION_MESSAGE_TYPE: i32 = 81667;

    const REQUEST_LISTENER_FLAGS_OFFSET: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;
    const REQUEST_LOCAL_ONLY_OFFSET: usize = Self::REQUEST_LISTENER_FLAGS_OFFSET + BitsUtil::INT_SIZE_IN_BYTES as usize;
    const REQUEST_INITIAL_FRAME_SIZE: usize = Self::REQUEST_LOCAL_ONLY_OFFSET + BitsUtil::BOOLEAN_SIZE_IN_BYTES as usize;
    const RESPONSE_RESPONSE_OFFSET: usize = ClientMessage::RESPONSE_BACKUP_ACKS_OFFSET as usize + BitsUtil::BYTE_SIZE_IN_BYTES as usize;
    const EVENT_I_MAP_INVALIDATION_SOURCE_UUID_OFFSET: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;
    const EVENT_I_MAP_INVALIDATION_PARTITION_UUID_OFFSET: usize = Self::EVENT_I_MAP_INVALIDATION_SOURCE_UUID_OFFSET as usize+ BitsUtil::UUID_SIZE_IN_BYTES as usize;
    const EVENT_I_MAP_INVALIDATION_SEQUENCE_OFFSET: usize = Self::EVENT_I_MAP_INVALIDATION_PARTITION_UUID_OFFSET as usize+ BitsUtil::UUID_SIZE_IN_BYTES as usize;

    pub fn encode_request<'a>(name: &'a String, listener_flags: &'a i32, local_only: &'a bool) -> Pin<Box<dyn Future<Output=ClientMessage> + Send + Sync + 'a>> {
        Box::pin(async move {
            let mut client_message = ClientMessage::create_for_encode().await;
            client_message.retryable = false;

            let initial_frame = Frame::create_initial_frame(Self::REQUEST_INITIAL_FRAME_SIZE, None);
            FixSizedTypesCodec::encode_int(&mut *initial_frame.content.lock().await, Self::REQUEST_LISTENER_FLAGS_OFFSET, listener_flags).await;
            FixSizedTypesCodec::encode_boolean(&mut *initial_frame.content.lock().await, Self::REQUEST_LOCAL_ONLY_OFFSET, local_only).await;
            client_message.add_frame(initial_frame).await;
            client_message.set_message_type(Self::REQUEST_MESSAGE_TYPE).await;
            client_message.set_partition_id(-1).await;

            StringCodec::encode(&mut client_message, name).await;

            client_message
        })
    }


    pub fn decode_response<'a>(client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=Uuid> + Send + Sync + 'a>> {
        Box::pin(async move {
            let initial_frame = client_message.next_frame().await.unwrap();

            let x = FixSizedTypesCodec::decode_uuid(&*initial_frame.content.lock().await, Self::RESPONSE_RESPONSE_OFFSET).await; x
        })
    }


    pub async fn handle(client_message: &mut ClientMessage, handle_i_map_invalidation_event: Option<Pin<Box<dyn Fn(Option<HeapData>, Uuid, Uuid, i64) -> Pin<Box<dyn Future<Output=()> + Send + Sync>> + Send + Sync>>>, handle_i_map_batch_invalidation_event: Option<Pin<Box<dyn Fn(Vec<HeapData>, UUID[], UUID[], Long[]) -> Pin<Box<dyn Future<Output=()> + Send + Sync>> + Send + Sync>>>) {
        let message_type = client_message.get_message_type().await;
        if message_type == Self::EVENT_I_MAP_INVALIDATION_MESSAGE_TYPE && handle_i_map_invalidation_event.is_some() {
            let initial_frame = client_message.next_frame().await.unwrap();
            let source_uuid = FixSizedTypesCodec::decode_uuid(&mut *initial_frame.content.lock().await, Self::EVENT_I_MAP_INVALIDATION_SOURCE_UUID_OFFSET).await;
            let partition_uuid = FixSizedTypesCodec::decode_uuid(&mut *initial_frame.content.lock().await, Self::EVENT_I_MAP_INVALIDATION_PARTITION_UUID_OFFSET).await;
            let sequence = FixSizedTypesCodec::decode_long(&mut *initial_frame.content.lock().await, Self::EVENT_I_MAP_INVALIDATION_SEQUENCE_OFFSET).await;
            let key = CodecUtil::decode_nullable(client_message, DataCodec::decode).await;
            handle_i_map_invalidation_event.unwrap()(key, source_uuid, partition_uuid, sequence).await;
            return;
        }
        if message_type == Self::EVENT_I_MAP_BATCH_INVALIDATION_MESSAGE_TYPE && handle_i_map_batch_invalidation_event.is_some() {
            // empty initial frame
            client_message.next_frame().await.unwrap();
            let keys = ListMultiFrameCodec::decode(client_message, DataCodec::decode).await;
            let source_uuids = ListUUIDCodec::decode(client_message).await;
            let partition_uuids = ListUUIDCodec::decode(client_message).await;
            let sequences = ListLongCodec::decode(client_message).await;
            handle_i_map_batch_invalidation_event.unwrap()(keys, source_uuids, partition_uuids, sequences).await;
            return;
        }
    }
}