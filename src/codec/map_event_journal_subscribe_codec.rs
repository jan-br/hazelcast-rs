use crate::codec_builtin::string_codec::StringCodec;

use std::mem::MaybeUninit;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::util::bits_util::BitsUtil;
use core::pin::Pin;
use std::future::Future;
use crate::codec_builtin::fix_sized_types_codec::FixSizedTypesCodec;


/** @internal */
#[derive(Default, Clone)]
pub struct MapEventJournalSubscribeResponseParams {
    pub oldest_sequence: i64,
    pub newest_sequence: i64,
}


pub struct MapEventJournalSubscribeCodec;

impl MapEventJournalSubscribeCodec {

    // hex: 0x014100
    const REQUEST_MESSAGE_TYPE: i32 = 82176;
    // hex: 0x014101
    // RESPONSE_MESSAGE_TYPE = 82177

    const REQUEST_INITIAL_FRAME_SIZE: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;
    const RESPONSE_OLDEST_SEQUENCE_OFFSET: usize = ClientMessage::RESPONSE_BACKUP_ACKS_OFFSET as usize + BitsUtil::BYTE_SIZE_IN_BYTES as usize;
    const RESPONSE_NEWEST_SEQUENCE_OFFSET: usize = Self::RESPONSE_OLDEST_SEQUENCE_OFFSET as usize + BitsUtil::LONG_SIZE_IN_BYTES as usize;

    pub fn encode_request<'a>(name: &'a String) -> Pin<Box<dyn Future<Output=ClientMessage> + Send + Sync + 'a>> {
        Box::pin(async move {
            let mut client_message = ClientMessage::create_for_encode().await;
            client_message.retryable = true;

            let initial_frame = Frame::create_initial_frame(Self::REQUEST_INITIAL_FRAME_SIZE, None);
            client_message.add_frame(initial_frame).await;
            client_message.set_message_type(Self::REQUEST_MESSAGE_TYPE).await;
            client_message.set_partition_id(-1).await;

            StringCodec::encode(&mut client_message, name).await;

            client_message
        })
    }

    pub fn decode_response<'a>(client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=MapEventJournalSubscribeResponseParams> + Send + Sync + 'a>> {
        Box::pin(async move {
            let initial_frame = client_message.next_frame().await.unwrap();
            #[allow(invalid_value)]
            let mut response = unsafe { MaybeUninit::<ClientAuthenticationResponseParams>::zeroed().assume_init() };

            response.oldest_sequence = FixSizedTypesCodec::decode_long(&*initial_frame.content.lock().await, Self::RESPONSE_OLDEST_SEQUENCE_OFFSET).await;
            response.newest_sequence = FixSizedTypesCodec::decode_long(&*initial_frame.content.lock().await, Self::RESPONSE_NEWEST_SEQUENCE_OFFSET).await;
            response
        })
    }

}