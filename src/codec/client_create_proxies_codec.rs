use crate::codec_builtin::entry_list_codec::EntryListCodec;
use crate::codec_builtin::string_codec::StringCodec;

use std::mem::MaybeUninit;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::util::bits_util::BitsUtil;
use core::pin::Pin;
use std::future::Future;



pub struct ClientCreateProxiesCodec;

impl ClientCreateProxiesCodec {

    // hex: 0x000E00
    const REQUEST_MESSAGE_TYPE: i32 = 3584;
    // hex: 0x000E01
    // RESPONSE_MESSAGE_TYPE = 3585

    const REQUEST_INITIAL_FRAME_SIZE: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;

    pub fn encode_request<'a>(proxies: &'a Vec<(String, String)>) -> Pin<Box<dyn Future<Output=ClientMessage> + Send + Sync + 'a>> {
        Box::pin(async move {
            let mut client_message = ClientMessage::create_for_encode().await;
            client_message.retryable = false;

            let initial_frame = Frame::create_initial_frame(Self::REQUEST_INITIAL_FRAME_SIZE, None);
            client_message.add_frame(initial_frame).await;
            client_message.set_message_type(Self::REQUEST_MESSAGE_TYPE).await;
            client_message.set_partition_id(-1).await;

            EntryListCodec::encode(&mut client_message, proxies, StringCodec::encode, StringCodec::encode).await;

            client_message
        })
    }


}