use crate::serialization::serializable::{IdentifiedDataSerializable, IdentifiedDataSerializableSerialization};

pub const PREDICATE_FACTORY_ID: i32 = -20;

pub fn predicate_factory(class_id: i32) -> Box<dyn IdentifiedDataSerializable> {
  todo!()
}