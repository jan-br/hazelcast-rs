use crate::codec_builtin::string_codec::StringCodec;
use crate::core::distributed_object_info::DistributedObjectInfo;

use crate::codec_builtin::list_multi_frame_codec::ListMultiFrameCodec;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::codec_builtin::codec_util::CodecUtil;
use core::pin::Pin;
use std::future::Future;

pub struct DistributedObjectInfoCodec;

impl DistributedObjectInfoCodec {

    pub fn encode<'a>(client_message: &'a mut ClientMessage, distributed_object_info: &'a DistributedObjectInfo) -> Pin<Box<dyn Future<Output=()> + Send + Sync + 'a>> {
        Box::pin(async move {
            client_message.add_frame(Frame::new_begin_frame()).await;

            StringCodec::encode(client_message, &distributed_object_info.service_name).await;
            StringCodec::encode(client_message, &distributed_object_info.name).await;

            client_message.add_frame(Frame::new_end_frame()).await;
        })
    }

    pub fn decode<'a>(client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=DistributedObjectInfo> + Send + Sync + 'a>> {
        Box::pin(async move {
            client_message.next_frame().await.unwrap();

            let service_name = StringCodec::decode(client_message).await;
            let name = StringCodec::decode(client_message).await;
            CodecUtil::fast_forward_to_end_frame(client_message).await;

            DistributedObjectInfo::new(service_name, name)
        })
    }
}