use std::collections::HashMap;
use uuid::Uuid;
use crate::core::member::version::MemberVersion;
use crate::connection::address::Address;
use crate::core::member::endpoint::EndpointQualifier;

pub struct MemberInfo {
  pub address: Address,
  pub uuid: Uuid,
  pub lite_member: bool,
  pub attributes: HashMap<String, String>,
  pub version: MemberVersion,
  pub address_map: HashMap<EndpointQualifier, Address>,
}

impl MemberInfo {
  pub fn new(
    address: Address,
    uuid: Uuid,
    attributes: HashMap<String, String>,
    lite_member: bool,
    version: MemberVersion,
    is_address_map_exists: bool,
    mut address_map: Option<HashMap<EndpointQualifier, Address>>,
  ) -> MemberInfo {
    let address_map = address_map.unwrap_or(HashMap::new());
    MemberInfo {
      address,
      uuid,
      lite_member,
      attributes,
      version,
      address_map: if is_address_map_exists {
        address_map
      } else {
        HashMap::new()
      },
    }
  }
}