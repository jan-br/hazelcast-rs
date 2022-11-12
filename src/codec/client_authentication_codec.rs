use uuid::Uuid;
use crate::codec_builtin::string_codec::StringCodec;
use crate::codec_builtin::codec_util::CodecUtil;
use crate::codec_builtin::list_multi_frame_codec::ListMultiFrameCodec;
use crate::connection::address::Address;
use crate::codec::custom::address_codec::AddressCodec;

use std::mem::MaybeUninit;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::util::bits_util::BitsUtil;
use core::pin::Pin;
use std::future::Future;
use crate::codec_builtin::fix_sized_types_codec::FixSizedTypesCodec;


/** @internal */
#[derive(Default, Clone)]
pub struct ClientAuthenticationResponseParams {
    pub status: u8,
    pub address: Option<Address>,
    pub member_uuid: Option<Uuid>,
    pub serialization_version: u8,
    pub server_hazelcast_version: String,
    pub partition_count: i32,
    pub cluster_id: Uuid,
    pub failover_supported: bool,
}


pub struct ClientAuthenticationCodec;

impl ClientAuthenticationCodec {

    // hex: 0x000100
    const REQUEST_MESSAGE_TYPE: i32 = 256;
    // hex: 0x000101
    // RESPONSE_MESSAGE_TYPE = 257

    const REQUEST_UUID_OFFSET: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;
    const REQUEST_SERIALIZATION_VERSION_OFFSET: usize = Self::REQUEST_UUID_OFFSET + BitsUtil::UUID_SIZE_IN_BYTES as usize;
    const REQUEST_INITIAL_FRAME_SIZE: usize = Self::REQUEST_SERIALIZATION_VERSION_OFFSET + BitsUtil::BYTE_SIZE_IN_BYTES as usize;
    const RESPONSE_STATUS_OFFSET: usize = ClientMessage::RESPONSE_BACKUP_ACKS_OFFSET as usize + BitsUtil::BYTE_SIZE_IN_BYTES as usize;
    const RESPONSE_MEMBER_UUID_OFFSET: usize = Self::RESPONSE_STATUS_OFFSET as usize + BitsUtil::BYTE_SIZE_IN_BYTES as usize;
    const RESPONSE_SERIALIZATION_VERSION_OFFSET: usize = Self::RESPONSE_MEMBER_UUID_OFFSET as usize + BitsUtil::UUID_SIZE_IN_BYTES as usize;
    const RESPONSE_PARTITION_COUNT_OFFSET: usize = Self::RESPONSE_SERIALIZATION_VERSION_OFFSET as usize + BitsUtil::BYTE_SIZE_IN_BYTES as usize;
    const RESPONSE_CLUSTER_ID_OFFSET: usize = Self::RESPONSE_PARTITION_COUNT_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;
    const RESPONSE_FAILOVER_SUPPORTED_OFFSET: usize = Self::RESPONSE_CLUSTER_ID_OFFSET as usize + BitsUtil::UUID_SIZE_IN_BYTES as usize;

    pub fn encode_request<'a>(cluster_name: &'a String, username: &'a Option<&'a String>, password: &'a Option<&'a String>, uuid: &'a Option<&'a Uuid>, client_type: &'a String, serialization_version: &'a u8, client_hazelcast_version: &'a String, client_name: &'a String, labels: &'a Vec<String>) -> Pin<Box<dyn Future<Output=ClientMessage> + Send + Sync + 'a>> {
        Box::pin(async move {
            let mut client_message = ClientMessage::create_for_encode().await;
            client_message.retryable = true;

            let initial_frame = Frame::create_initial_frame(Self::REQUEST_INITIAL_FRAME_SIZE, None);
            FixSizedTypesCodec::encode_uuid_nullable(&mut *initial_frame.content.lock().await, Self::REQUEST_UUID_OFFSET, uuid).await;
            FixSizedTypesCodec::encode_byte(&mut *initial_frame.content.lock().await, Self::REQUEST_SERIALIZATION_VERSION_OFFSET, serialization_version).await;
            client_message.add_frame(initial_frame).await;
            client_message.set_message_type(Self::REQUEST_MESSAGE_TYPE).await;
            client_message.set_partition_id(-1).await;

            StringCodec::encode(&mut client_message, cluster_name).await;
            CodecUtil::encode_nullable(&mut client_message, username, StringCodec::encode).await;
            CodecUtil::encode_nullable(&mut client_message, password, StringCodec::encode).await;
            StringCodec::encode(&mut client_message, client_type).await;
            StringCodec::encode(&mut client_message, client_hazelcast_version).await;
            StringCodec::encode(&mut client_message, client_name).await;
            ListMultiFrameCodec::encode(&mut client_message, labels, StringCodec::encode).await;

            client_message
        })
    }

    pub fn decode_response<'a>(client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=ClientAuthenticationResponseParams> + Send + Sync + 'a>> {
        Box::pin(async move {
            let initial_frame = client_message.next_frame().await.unwrap();
            #[allow(invalid_value)]
            let mut response = unsafe { MaybeUninit::<ClientAuthenticationResponseParams>::zeroed().assume_init() };

            response.status = FixSizedTypesCodec::decode_byte(&*initial_frame.content.lock().await, Self::RESPONSE_STATUS_OFFSET).await;
            response.member_uuid = FixSizedTypesCodec::decode_uuid_nullable(&*initial_frame.content.lock().await, Self::RESPONSE_MEMBER_UUID_OFFSET).await;
            response.serialization_version = FixSizedTypesCodec::decode_byte(&*initial_frame.content.lock().await, Self::RESPONSE_SERIALIZATION_VERSION_OFFSET).await;
            response.partition_count = FixSizedTypesCodec::decode_int(&*initial_frame.content.lock().await, Self::RESPONSE_PARTITION_COUNT_OFFSET).await;
            response.cluster_id = FixSizedTypesCodec::decode_uuid(&*initial_frame.content.lock().await, Self::RESPONSE_CLUSTER_ID_OFFSET).await;
            response.failover_supported = FixSizedTypesCodec::decode_boolean(&*initial_frame.content.lock().await, Self::RESPONSE_FAILOVER_SUPPORTED_OFFSET).await;
            response.address = CodecUtil::decode_nullable(client_message, AddressCodec::decode).await;
            response.server_hazelcast_version = StringCodec::decode(client_message).await;
            response
        })
    }

}