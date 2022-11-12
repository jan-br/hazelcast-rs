use crate::core::member::info::MemberInfo;
use crate::codec_builtin::list_multi_frame_codec::ListMultiFrameCodec;
use crate::codec::custom::member_info_codec::MemberInfoCodec;
use std::vec::Vec;
use crate::codec_builtin::entry_list_uuid_list_integer_codec::EntryListUUIDListIntegerCodec;
use uuid::Uuid;

use std::mem::MaybeUninit;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::util::bits_util::BitsUtil;
use core::pin::Pin;
use std::future::Future;
use crate::codec_builtin::fix_sized_types_codec::FixSizedTypesCodec;



pub struct ClientAddClusterViewListenerCodec;

impl ClientAddClusterViewListenerCodec {

    // hex: 0x000300
    const REQUEST_MESSAGE_TYPE: i32 = 768;
    // hex: 0x000301
    // RESPONSE_MESSAGE_TYPE = 769
    // hex: 0x000302
    const EVENT_MEMBERS_VIEW_MESSAGE_TYPE: i32 = 770;
    // hex: 0x000303
    const EVENT_PARTITIONS_VIEW_MESSAGE_TYPE: i32 = 771;

    const REQUEST_INITIAL_FRAME_SIZE: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;
    const EVENT_MEMBERS_VIEW_VERSION_OFFSET: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;
    const EVENT_PARTITIONS_VIEW_VERSION_OFFSET: usize = ClientMessage::PARTITION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize;

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


    pub async fn handle(client_message: &mut ClientMessage, handle_members_view_event: Option<impl Fn(i32, Vec<MemberInfo>)>, handle_partitions_view_event: Option<impl Fn(i32, Vec<(Uuid, Vec<i32>)>)>) {
        let message_type = client_message.get_message_type().await;
        if message_type == Self::EVENT_MEMBERS_VIEW_MESSAGE_TYPE && handle_members_view_event.is_some() {
            let initial_frame = client_message.next_frame().await.unwrap();
            let version = FixSizedTypesCodec::decode_int(&mut *initial_frame.content.lock().await, Self::EVENT_MEMBERS_VIEW_VERSION_OFFSET).await;
            let member_infos = ListMultiFrameCodec::decode(client_message, MemberInfoCodec::decode).await;
            handle_members_view_event.unwrap()(version, member_infos);
            return;
        }
        if message_type == Self::EVENT_PARTITIONS_VIEW_MESSAGE_TYPE && handle_partitions_view_event.is_some() {
            let initial_frame = client_message.next_frame().await.unwrap();
            let version = FixSizedTypesCodec::decode_int(&mut *initial_frame.content.lock().await, Self::EVENT_PARTITIONS_VIEW_VERSION_OFFSET).await;
            let partitions = EntryListUUIDListIntegerCodec::decode(client_message).await;
            handle_partitions_view_event.unwrap()(version, partitions);
            return;
        }
    }
}