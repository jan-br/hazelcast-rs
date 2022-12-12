use uuid::Uuid;

use std::mem::MaybeUninit;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::util::bits_util::BitsUtil;
use core::pin::Pin;
use std::future::Future;
use crate::codec_builtin::fix_sized_types_codec::FixSizedTypesCodec;



pub struct ClientLocalBackupListenerCodec;

impl ClientLocalBackupListenerCodec {

    // hex: 0x000F00
    const REQUEST_MESSAGE_TYPE: i32 = 3840;
    // hex: 0x000F01
    // RESPONSE_MESSAGE_TYPE = 3841
    // hex: 0x000F02
    const EVENT_BACKUP_MESSAGE_TYPE: i32 = 3842;

    const REQUEST_INITIAL_FRAME_SIZE: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;
    const RESPONSE_RESPONSE_OFFSET: usize = ClientMessage::RESPONSE_BACKUP_ACKS_OFFSET as usize + BitsUtil::BYTE_SIZE_IN_BYTES as usize;
    const EVENT_BACKUP_SOURCE_INVOCATION_CORRELATION_ID_OFFSET: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;

    pub fn encode_request<'a>() -> Pin<Box<dyn Future<Output=ClientMessage> + Send + Sync + 'a>> {
        Box::pin(async move {
            let mut client_message = ClientMessage::create_for_encode().await;
            client_message.retryable = false;

            let initial_frame = Frame::create_initial_frame(Self::REQUEST_INITIAL_FRAME_SIZE, None);
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


    pub async fn handle(client_message: &mut ClientMessage, handle_backup_event: Option<Pin<Box<dyn Fn(i64) -> Pin<Box<dyn Future<Output=()> + Send + Sync>> + Send + Sync>>>) {
        let message_type = client_message.get_message_type().await;
        if message_type == Self::EVENT_BACKUP_MESSAGE_TYPE && handle_backup_event.is_some() {
            let initial_frame = client_message.next_frame().await.unwrap();
            let source_invocation_correlation_id = FixSizedTypesCodec::decode_long(&mut *initial_frame.content.lock().await, Self::EVENT_BACKUP_SOURCE_INVOCATION_CORRELATION_ID_OFFSET).await;
            handle_backup_event.unwrap()(source_invocation_correlation_id).await;
            return;
        }
    }
}