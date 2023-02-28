pub mod default;
pub mod json;

use std::any::Any;
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializable::{IdentifiedDataSerializable, IdentifiedDataSerializableSerialization};

pub trait Serializer<T: Any>: Send + Sync {
  fn id(&self) -> i32;
  fn read(&self, input: &mut ObjectDataInput) -> T;
  fn write(&self, output: &mut ObjectDataOutput, object: T);
}