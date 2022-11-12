use crate::serialization::schema::Schema;
use crate::codec::custom::schema_codec::SchemaCodec;
use crate::codec_builtin::codec_util::CodecUtil;

use std::mem::MaybeUninit;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::util::bits_util::BitsUtil;
use core::pin::Pin;
use std::future::Future;
use crate::codec_builtin::fix_sized_types_codec::FixSizedTypesCodec;



pub struct ClientFetchSchemaCodec;

impl ClientFetchSchemaCodec {

    // hex: 0x001400
    const REQUEST_MESSAGE_TYPE: i32 = 5120;
    // hex: 0x001401
    // RESPONSE_MESSAGE_TYPE = 5121

    const REQUEST_SCHEMA_ID_OFFSET: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;
    const REQUEST_INITIAL_FRAME_SIZE: usize = Self::REQUEST_SCHEMA_ID_OFFSET + BitsUtil::LONG_SIZE_IN_BYTES as usize;

    pub fn encode_request<'a>(schema_id: &'a i64) -> Pin<Box<dyn Future<Output=ClientMessage> + Send + Sync + 'a>> {
        Box::pin(async move {
            let mut client_message = ClientMessage::create_for_encode().await;
            client_message.retryable = true;

            let initial_frame = Frame::create_initial_frame(Self::REQUEST_INITIAL_FRAME_SIZE, None);
            FixSizedTypesCodec::encode_long(&mut *initial_frame.content.lock().await, Self::REQUEST_SCHEMA_ID_OFFSET, schema_id).await;
            client_message.add_frame(initial_frame).await;
            client_message.set_message_type(Self::REQUEST_MESSAGE_TYPE).await;
            client_message.set_partition_id(-1).await;


            client_message
        })
    }


    pub fn decode_response<'a>(client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=Option<Schema>> + Send + Sync + 'a>> {
        Box::pin(async move {
            // empty initial frame
            client_message.next_frame().await.unwrap();

            CodecUtil::decode_nullable(client_message, SchemaCodec::decode).await
        })
    }


}