use crate::serialization::serializable::{IdentifiedDataSerializable, IdentifiedDataSerializableSerialization};

pub const REST_VALUE_FACTORY_ID :i32= - 25;
pub const REST_VALUE_CLASS_ID: i32 = 1;

pub fn rest_value_factory(class_id: i32) -> Box<dyn IdentifiedDataSerializable> {
  todo!()
}