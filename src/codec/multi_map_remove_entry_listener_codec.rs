use uuid::Uuid;
use crate::codec_builtin::string_codec::StringCodec;

use std::mem::MaybeUninit;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::util::bits_util::BitsUtil;
use core::pin::Pin;
use std::future::Future;
use crate::codec_builtin::fix_sized_types_codec::FixSizedTypesCodec;



pub struct MultiMapRemoveEntryListenerCodec;

impl MultiMapRemoveEntryListenerCodec {

    // hex: 0x020F00
    const REQUEST_MESSAGE_TYPE: i32 = 134912;
    // hex: 0x020F01
    // RESPONSE_MESSAGE_TYPE = 134913

    const REQUEST_REGISTRATION_ID_OFFSET: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;
    const REQUEST_INITIAL_FRAME_SIZE: usize = Self::REQUEST_REGISTRATION_ID_OFFSET + BitsUtil::UUID_SIZE_IN_BYTES as usize;
    const RESPONSE_RESPONSE_OFFSET: usize = ClientMessage::RESPONSE_BACKUP_ACKS_OFFSET as usize + BitsUtil::BYTE_SIZE_IN_BYTES as usize;

    pub fn encode_request<'a>(name: &'a String, registration_id: &'a Uuid) -> Pin<Box<dyn Future<Output=ClientMessage> + Send + Sync + 'a>> {
        Box::pin(async move {
            let mut client_message = ClientMessage::create_for_encode().await;
            client_message.retryable = true;

            let initial_frame = Frame::create_initial_frame(Self::REQUEST_INITIAL_FRAME_SIZE, None);
            FixSizedTypesCodec::encode_uuid(&mut *initial_frame.content.lock().await, Self::REQUEST_REGISTRATION_ID_OFFSET, registration_id).await;
            client_message.add_frame(initial_frame).await;
            client_message.set_message_type(Self::REQUEST_MESSAGE_TYPE).await;
            client_message.set_partition_id(-1).await;

            StringCodec::encode(&mut client_message, name).await;

            client_message
        })
    }


    pub fn decode_response<'a>(client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=bool> + Send + Sync + 'a>> {
        Box::pin(async move {
            let initial_frame = client_message.next_frame().await.unwrap();

            let x = FixSizedTypesCodec::decode_boolean(&*initial_frame.content.lock().await, Self::RESPONSE_RESPONSE_OFFSET).await; x
        })
    }


}