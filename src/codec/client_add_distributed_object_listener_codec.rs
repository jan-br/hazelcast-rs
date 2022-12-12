use uuid::Uuid;
use crate::codec_builtin::string_codec::StringCodec;

use std::mem::MaybeUninit;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::util::bits_util::BitsUtil;
use core::pin::Pin;
use std::future::Future;
use crate::codec_builtin::fix_sized_types_codec::FixSizedTypesCodec;



pub struct ClientAddDistributedObjectListenerCodec;

impl ClientAddDistributedObjectListenerCodec {

    // hex: 0x000900
    const REQUEST_MESSAGE_TYPE: i32 = 2304;
    // hex: 0x000901
    // RESPONSE_MESSAGE_TYPE = 2305
    // hex: 0x000902
    const EVENT_DISTRIBUTED_OBJECT_MESSAGE_TYPE: i32 = 2306;

    const REQUEST_LOCAL_ONLY_OFFSET: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;
    const REQUEST_INITIAL_FRAME_SIZE: usize = Self::REQUEST_LOCAL_ONLY_OFFSET + BitsUtil::BOOLEAN_SIZE_IN_BYTES as usize;
    const RESPONSE_RESPONSE_OFFSET: usize = ClientMessage::RESPONSE_BACKUP_ACKS_OFFSET as usize + BitsUtil::BYTE_SIZE_IN_BYTES as usize;
    const EVENT_DISTRIBUTED_OBJECT_SOURCE_OFFSET: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;

    pub fn encode_request<'a>(local_only: &'a bool) -> Pin<Box<dyn Future<Output=ClientMessage> + Send + Sync + 'a>> {
        Box::pin(async move {
            let mut client_message = ClientMessage::create_for_encode().await;
            client_message.retryable = false;

            let initial_frame = Frame::create_initial_frame(Self::REQUEST_INITIAL_FRAME_SIZE, None);
            FixSizedTypesCodec::encode_boolean(&mut *initial_frame.content.lock().await, Self::REQUEST_LOCAL_ONLY_OFFSET, local_only).await;
            client_message.add_frame(initial_frame).await;
            client_message.set_message_type(Self::REQUEST_MESSAGE_TYPE).await;
            client_message.set_partition_id(-1).await;


            client_message
        })
    }


    pub fn decode_response<'a>(client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=Uuid> + Send + Sync + 'a>> {
        Box::pin(async move {
            let initial_frame = client_message.next_frame().await.unwrap();

            let x = FixSizedTypesCodec::decode_uuid(&*initial_frame.content.lock().await, Self::RESPONSE_RESPONSE_OFFSET).await; x
        })
    }


    pub async fn handle(client_message: &mut ClientMessage, handle_distributed_object_event: Option<Pin<Box<dyn Fn(String, String, String, Uuid) -> Pin<Box<dyn Future<Output=()> + Send + Sync>> + Send + Sync>>>) {
        let message_type = client_message.get_message_type().await;
        if message_type == Self::EVENT_DISTRIBUTED_OBJECT_MESSAGE_TYPE && handle_distributed_object_event.is_some() {
            let initial_frame = client_message.next_frame().await.unwrap();
            let source = FixSizedTypesCodec::decode_uuid(&mut *initial_frame.content.lock().await, Self::EVENT_DISTRIBUTED_OBJECT_SOURCE_OFFSET).await;
            let name = StringCodec::decode(client_message).await;
            let service_name = StringCodec::decode(client_message).await;
            let event_type = StringCodec::decode(client_message).await;
            handle_distributed_object_event.unwrap()(name, service_name, event_type, source).await;
            return;
        }
    }
}