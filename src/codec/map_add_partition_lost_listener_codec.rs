use crate::codec_builtin::string_codec::StringCodec;
use uuid::Uuid;

use std::mem::MaybeUninit;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::util::bits_util::BitsUtil;
use core::pin::Pin;
use std::future::Future;
use crate::codec_builtin::fix_sized_types_codec::FixSizedTypesCodec;



pub struct MapAddPartitionLostListenerCodec;

impl MapAddPartitionLostListenerCodec {

    // hex: 0x011B00
    const REQUEST_MESSAGE_TYPE: i32 = 72448;
    // hex: 0x011B01
    // RESPONSE_MESSAGE_TYPE = 72449
    // hex: 0x011B02
    const EVENT_MAP_PARTITION_LOST_MESSAGE_TYPE: i32 = 72450;

    const REQUEST_LOCAL_ONLY_OFFSET: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;
    const REQUEST_INITIAL_FRAME_SIZE: usize = Self::REQUEST_LOCAL_ONLY_OFFSET + BitsUtil::BOOLEAN_SIZE_IN_BYTES as usize;
    const RESPONSE_RESPONSE_OFFSET: usize = ClientMessage::RESPONSE_BACKUP_ACKS_OFFSET as usize + BitsUtil::BYTE_SIZE_IN_BYTES as usize;
    const EVENT_MAP_PARTITION_LOST_PARTITION_ID_OFFSET: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;
    const EVENT_MAP_PARTITION_LOST_UUID_OFFSET: usize = EVENT_MAP_PARTITION_LOST_PARTITION_ID_OFFSET as usize+ BitsUtil::INT_SIZE_IN_BYTES as usize;

    pub fn encode_request<'a>(name: &'a String, local_only: &'a bool) -> Pin<Box<dyn Future<Output=ClientMessage> + Send + Sync + 'a>> {
        Box::pin(async move {
            let mut client_message = ClientMessage::create_for_encode().await;
            client_message.retryable = false;

            let initial_frame = Frame::create_initial_frame(Self::REQUEST_INITIAL_FRAME_SIZE, None);
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

            FixSizedTypesCodec::decode_uuid(&*initial_frame.content.lock().await, Self::RESPONSE_RESPONSE_OFFSET).await
        })
    }


    pub async fn handle(client_message: &mut ClientMessage, handle_map_partition_lost_event: Option<impl Fn(i32, Uuid)>) {
        let message_type = client_message.get_message_type().await;
        if message_type == Self::EVENT_MAP_PARTITION_LOST_MESSAGE_TYPE && handle_map_partition_lost_event.is_some() {
            let initial_frame = client_message.next_frame().await.unwrap();
            let partition_id = FixSizedTypesCodec::decode_int(&mut *initial_frame.content.lock().await, Self::EVENT_MAP_PARTITION_LOST_PARTITION_ID_OFFSET).await;
            let uuid = FixSizedTypesCodec::decode_uuid(&mut *initial_frame.content.lock().await, Self::EVENT_MAP_PARTITION_LOST_UUID_OFFSET).await;
            handle_map_partition_lost_event.unwrap()(partition_id, uuid);
            return;
        }
    }
}