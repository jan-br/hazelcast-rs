use uuid::Uuid;
use crate::codec_builtin::list_multi_frame_codec::ListMultiFrameCodec;
use crate::codec_builtin::string_codec::StringCodec;
use crate::codec_builtin::entry_list_codec::EntryListCodec;
use crate::codec_builtin::codec_builtin::EntryListIntegerLongCodec;
use crate::codec_builtin::codec_builtin::EntryListIntegerUUIDCodec;

use std::mem::MaybeUninit;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::util::bits_util::BitsUtil;
use core::pin::Pin;
use std::future::Future;
use crate::codec_builtin::fix_sized_types_codec::FixSizedTypesCodec;


/** @internal */
#[derive(Default, Clone)]
pub struct MapFetchNearCacheInvalidationMetadataResponseParams {
    pub name_partition_sequence_list: Array<[string, Array<[number, Long]>]>,
    pub partition_uuid_list: Array<[number, UUID]>,
}


pub struct MapFetchNearCacheInvalidationMetadataCodec;

impl MapFetchNearCacheInvalidationMetadataCodec {

    // hex: 0x013D00
    const REQUEST_MESSAGE_TYPE: i32 = 81152;
    // hex: 0x013D01
    // RESPONSE_MESSAGE_TYPE = 81153

    const REQUEST_UUID_OFFSET: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;
    const REQUEST_INITIAL_FRAME_SIZE: usize = Self::REQUEST_UUID_OFFSET + BitsUtil::UUID_SIZE_IN_BYTES as usize;

    pub fn encode_request<'a>(names: &'a Vec<String>, uuid: &'a Uuid) -> Pin<Box<dyn Future<Output=ClientMessage> + Send + Sync + 'a>> {
        Box::pin(async move {
            let mut client_message = ClientMessage::create_for_encode().await;
            client_message.retryable = false;

            let initial_frame = Frame::create_initial_frame(Self::REQUEST_INITIAL_FRAME_SIZE, None);
            FixSizedTypesCodec::encode_uuid(&mut *initial_frame.content.lock().await, Self::REQUEST_UUID_OFFSET, uuid).await;
            client_message.add_frame(initial_frame).await;
            client_message.set_message_type(Self::REQUEST_MESSAGE_TYPE).await;
            client_message.set_partition_id(-1).await;

            ListMultiFrameCodec::encode(&mut client_message, names, StringCodec::encode).await;

            client_message
        })
    }

    pub fn decode_response<'a>(client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=MapFetchNearCacheInvalidationMetadataResponseParams> + Send + Sync + 'a>> {
        Box::pin(async move {
            // empty initial frame
            client_message.next_frame().await.unwrap();
            #[allow(invalid_value)]
            let mut response = unsafe { MaybeUninit::<ClientAuthenticationResponseParams>::zeroed().assume_init() };

            response.name_partition_sequence_list = EntryListCodec::decode(client_message, StringCodec::decode, EntryListIntegerLongCodec::decode).await;
            response.partition_uuid_list = EntryListIntegerUUIDCodec::decode(client_message).await;
            response
        })
    }

}