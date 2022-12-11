use crate::codec_builtin::string_codec::StringCodec;
use crate::codec_builtin::entry_list_codec::EntryListCodec;
use crate::codec_builtin::data_codec::DataCodec;
use crate::serialization::heap_data::HeapData;

use std::mem::MaybeUninit;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::util::bits_util::BitsUtil;
use core::pin::Pin;
use std::future::Future;
use crate::codec_builtin::fix_sized_types_codec::FixSizedTypesCodec;



pub struct MapPutAllCodec;

impl MapPutAllCodec {

    // hex: 0x012C00
    const REQUEST_MESSAGE_TYPE: i32 = 76800;
    // hex: 0x012C01
    // RESPONSE_MESSAGE_TYPE = 76801

    const REQUEST_TRIGGER_MAP_LOADER_OFFSET: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;
    const REQUEST_INITIAL_FRAME_SIZE: usize = Self::REQUEST_TRIGGER_MAP_LOADER_OFFSET + BitsUtil::BOOLEAN_SIZE_IN_BYTES as usize;

    pub fn encode_request<'a>(name: &'a String, entries: &'a Array<[Data, Data]>, trigger_map_loader: &'a bool) -> Pin<Box<dyn Future<Output=ClientMessage> + Send + Sync + 'a>> {
        Box::pin(async move {
            let mut client_message = ClientMessage::create_for_encode().await;
            client_message.retryable = false;

            let initial_frame = Frame::create_initial_frame(Self::REQUEST_INITIAL_FRAME_SIZE, None);
            FixSizedTypesCodec::encode_boolean(&mut *initial_frame.content.lock().await, Self::REQUEST_TRIGGER_MAP_LOADER_OFFSET, trigger_map_loader).await;
            client_message.add_frame(initial_frame).await;
            client_message.set_message_type(Self::REQUEST_MESSAGE_TYPE).await;
            client_message.set_partition_id(-1).await;

            StringCodec::encode(&mut client_message, name).await;
            EntryListCodec::encode(&mut client_message, entries, DataCodec::encode, DataCodec::encode).await;

            client_message
        })
    }


}