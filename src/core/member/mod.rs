pub mod version;
pub mod endpoint;
pub mod info;

use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use crate::core::member::version::MemberVersion;
use crate::connection::address::Address;
use crate::core::member::endpoint::EndpointQualifier;
use crate::core::member::info::MemberInfo;

pub struct Member {
  pub address: Address,
  pub uuid: Uuid,
  pub lite_member: bool,
  pub attributes: HashMap<String, String>,
  pub version: MemberVersion,
  pub address_map: HashMap<EndpointQualifier, Address>
}

impl Member {
  pub fn new(
    address: Address,
    uuid: Uuid,
    attributes: HashMap<String, String>,
    lite_member: bool,
    is_address_map_exists: bool,
    version: MemberVersion,
    address_map: HashMap<EndpointQualifier, Address>,
  ) -> MemberInfo {
    MemberInfo {
      address,
      uuid,
      lite_member,
      attributes,
      version,
      address_map
    }
  }
}

pub struct MemberListSnapshot {
  pub version: i32,
  pub members: HashMap<String, Arc<Member>>,
  pub member_list: Vec<Arc<Member>>,
}

pub type MemberSelector = fn(member: &Member) -> bool;