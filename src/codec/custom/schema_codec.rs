use crate::codec_builtin::string_codec::StringCodec;
use crate::serialization::schema::Schema;
use crate::codec::custom::field_descriptor_codec::FieldDescriptorCodec;

use crate::codec_builtin::list_multi_frame_codec::ListMultiFrameCodec;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::codec_builtin::codec_util::CodecUtil;
use core::pin::Pin;
use std::future::Future;

pub struct SchemaCodec;

impl SchemaCodec {

    pub fn encode<'a>(client_message: &'a mut ClientMessage, schema: &'a Schema) -> Pin<Box<dyn Future<Output=()> + Send + Sync + 'a>> {
        Box::pin(async move {
            client_message.add_frame(Frame::new_begin_frame()).await;

            StringCodec::encode(client_message, &schema.type_name).await;
            ListMultiFrameCodec::encode(client_message, &schema.fields, FieldDescriptorCodec::encode).await;

            client_message.add_frame(Frame::new_end_frame()).await;
        })
    }

    pub fn decode<'a>(client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=Schema> + Send + Sync + 'a>> {
        Box::pin(async move {
            client_message.next_frame().await.unwrap();

            let type_name = StringCodec::decode(client_message).await;
            let fields = ListMultiFrameCodec::decode(client_message, FieldDescriptorCodec::decode).await;
            CodecUtil::fast_forward_to_end_frame(client_message).await;

            Schema::new(type_name, fields)
        })
    }
}