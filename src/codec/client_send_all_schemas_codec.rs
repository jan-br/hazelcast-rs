
use std::mem::MaybeUninit;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::util::bits_util::BitsUtil;
use core::pin::Pin;
use std::future::Future;



pub struct ClientSendAllSchemasCodec;

impl ClientSendAllSchemasCodec {

    // hex: 0x001500
    const REQUEST_MESSAGE_TYPE: i32 = 5376;
    // hex: 0x001501
    // RESPONSE_MESSAGE_TYPE = 5377

    const REQUEST_INITIAL_FRAME_SIZE: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;

    pub fn encode_request<'a>(schemas: &'a Vec<Schema>) -> Pin<Box<dyn Future<Output=ClientMessage> + Send + Sync + 'a>> {
        Box::pin(async move {
            let mut client_message = ClientMessage::create_for_encode().await;
            client_message.retryable = true;

            let initial_frame = Frame::create_initial_frame(Self::REQUEST_INITIAL_FRAME_SIZE, None);
            client_message.add_frame(initial_frame).await;
            client_message.set_message_type(Self::REQUEST_MESSAGE_TYPE).await;
            client_message.set_partition_id(-1).await;

            ListMultiFrameCodec::encode(&mut client_message, schemas, SchemaCodec::encode).await;

            client_message
        })
    }


}