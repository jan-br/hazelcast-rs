use crate::core::distributed_object_info::DistributedObjectInfo;
use crate::codec_builtin::list_multi_frame_codec::ListMultiFrameCodec;
use crate::codec::custom::DistributedObjectInfoCodec::DistributedObjectInfoCodec;

use std::mem::MaybeUninit;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::util::bits_util::BitsUtil;
use core::pin::Pin;
use std::future::Future;



pub struct ClientGetDistributedObjectsCodec;

impl ClientGetDistributedObjectsCodec {

    // hex: 0x000800
    const REQUEST_MESSAGE_TYPE: i32 = 2048;
    // hex: 0x000801
    // RESPONSE_MESSAGE_TYPE = 2049

    const REQUEST_INITIAL_FRAME_SIZE: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;

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


    pub fn decode_response<'a>(client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=Vec<DistributedObjectInfo>> + Send + Sync + 'a>> {
        Box::pin(async move {
            // empty initial frame
            client_message.next_frame().await.unwrap();

            ListMultiFrameCodec::decode(client_message, DistributedObjectInfoCodec::decode).await
        })
    }


}