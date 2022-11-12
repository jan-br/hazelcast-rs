use std::any::Any;
use crate::serialization::data::{DataInput, DataInputReadObject};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializable::IdentifiedDataSerializable;
use crate::serialization::serializer::Serializer;

#[derive(Default)]
pub struct JavaArraySerializer;


impl Serializer<Box<Vec<Box<dyn Any>>>> for JavaArraySerializer {
  fn id(&self) -> i32 {
    -28
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<Vec<Box<dyn Any>>> {
    let size = input.read_int();
    let mut result = vec![];
    for _ in 0..size {
      result.push(input.read_object());
    }
    result.into()
  }

  fn write(&self, output: &mut ObjectDataOutput, object: Box<Vec<Box<dyn Any>>>) {
    //noop
  }
}