use std::pin::Pin;

use futures::Future;

use crate::codec::custom::address_codec::AddressCodec;
use crate::codec::custom::endpoint_qualifier_codec::EndpointQualifierCodec;
use crate::codec::custom::member_version_codec::MemberVersionCodec;
use crate::codec_builtin::codec_util::CodecUtil;
use crate::codec_builtin::fix_sized_types_codec::FixSizedTypesCodec;
use crate::codec_builtin::map_codec::MapCodec;
use crate::codec_builtin::string_codec::StringCodec;
use crate::core::member::info::MemberInfo;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::util::bits_util::BitsUtil;

pub struct MemberInfoCodec {}

impl MemberInfoCodec {
  const UUID_OFFSET: i32 = 0;
  const LITE_MEMBER_OFFSET: i32 = Self::UUID_OFFSET + BitsUtil::UUID_SIZE_IN_BYTES;
  const INITIAL_FRAME_SIZE: i32 = Self::LITE_MEMBER_OFFSET + BitsUtil::BOOLEAN_SIZE_IN_BYTES;

  pub fn encode<'a>(client_message: &'a mut ClientMessage, member_info: &'a MemberInfo) -> Pin<Box<dyn Future<Output=()> + Send + Sync + 'a>> {
    Box::pin(async move {
      client_message.add_frame(Frame::new_begin_frame().copy()).await;
      let mut initial_frame = Frame::create_initial_frame(Self::INITIAL_FRAME_SIZE as usize, Some(Frame::DEFAULT_FLAGS));
      FixSizedTypesCodec::encode_uuid(&mut *initial_frame.content.lock().await, Self::UUID_OFFSET as usize, &member_info.uuid).await;
      FixSizedTypesCodec::encode_boolean(&mut *initial_frame.content.lock().await, Self::LITE_MEMBER_OFFSET as usize, &member_info.lite_member).await;
      client_message.add_frame(initial_frame).await;

      AddressCodec::encode(client_message, &member_info.address).await;
      MapCodec::encode(
        client_message,
        &member_info.attributes,
        StringCodec::encode,
        StringCodec::encode,
      ).await;
      MemberVersionCodec::encode(client_message, &member_info.version).await;
    })
  }

  pub fn decode<'a>(client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=MemberInfo> + 'a>> {
    Box::pin(async move {
      client_message.next_frame().await.unwrap();

      let mut initial_frame = client_message.next_frame().await.unwrap();
      let uuid = FixSizedTypesCodec::decode_uuid(&mut *initial_frame.content.lock().await, Self::UUID_OFFSET as usize).await;
      let lite_member = FixSizedTypesCodec::decode_boolean(&mut *initial_frame.content.lock().await, Self::LITE_MEMBER_OFFSET as usize).await;

      let address = AddressCodec::decode(client_message).await;
      let attributes = MapCodec::decode::<String, String>(
        client_message,
        |client_message| Box::pin(StringCodec::decode(client_message)),
        |client_message| Box::pin(StringCodec::decode(client_message)),
      ).await;
      let version = MemberVersionCodec::decode(client_message).await;
      let mut is_address_map_exists = false;
      let mut address_map = None;
      if !client_message.peek_next_frame().unwrap().is_end_frame().await {
        address_map = Some(MapCodec::decode(client_message, |client_message| EndpointQualifierCodec::decode(client_message), |client_message| AddressCodec::decode(client_message)).await);
        is_address_map_exists = true;
      }
      CodecUtil::fast_forward_to_end_frame(client_message).await;

      MemberInfo::new(address, uuid, attributes, lite_member, version, is_address_map_exists, address_map)
    })
  }
}