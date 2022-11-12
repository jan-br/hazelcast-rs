use std::any::Any;
use std::fmt::{Display, Formatter};
use std::net::IpAddr;
use std::sync::Arc;
use crate::serialization::cluster_data_factory::{CLUSTER_DATA_ADDRESS_CLASS_ID, CLUSTER_DATA_FACTORY_ID};
use crate::serialization::data::{DataInput, DataOutput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializable::{IdentifiedDataSerializableSerialization, IdentifiedDataSerializableInfo};

pub mod provider;

#[derive(Debug, Clone)]
pub struct Addresses {
  pub primary: Vec<Arc<Address>>,
  pub secondary: Vec<Arc<Address>>,
}

impl Addresses {
  pub fn new() -> Addresses {
    Addresses {
      primary: Vec::new(),
      secondary: Vec::new(),
    }
  }

  pub fn add_all(&mut self, addresses: Self) {
    self.primary.extend(addresses.primary);
    self.secondary.extend(addresses.secondary);
  }
}

#[derive(Debug, Clone)]
pub struct Address {
  pub host: String,
  pub port: i32,
  pub scope: Option<u8>,
}

impl Address {
  pub fn new(host: impl Into<Option<String>>, port: impl Into<Option<i32>>) -> Address {
    Address {
      host: host.into().unwrap_or_default(),
      port: port.into().unwrap_or_default(),
      scope: Some(0),
    }
  }
}

impl IdentifiedDataSerializableInfo for Address {
  fn factory_id(&self) -> i32 {
    CLUSTER_DATA_FACTORY_ID
  }

  fn class_id(&self) -> i32 {
    CLUSTER_DATA_ADDRESS_CLASS_ID
  }
}

impl IdentifiedDataSerializableSerialization for Address {
  fn read_data(&mut self, input: &mut ObjectDataInput) {
    self.port = input.read_int();
    self.scope = Some(input.read_byte());
    self.host = input.read_string().map(|x| x.parse().unwrap()).unwrap();
  }

  fn write_data(&mut self, output: &mut ObjectDataOutput) {
    output.write_int(self.port);
    output.write_byte(self.scope.unwrap());
    output.write_string(Some(&self.host));
  }
}

impl PartialEq<Self> for Address {
  fn eq(&self, other: &Self) -> bool {
    self.host == other.host && self.port == other.port && self.scope == other.scope
  }
}

impl Eq for Address {}

impl Display for Address {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}:{:?}", self.host, self.port)
  }
}
