use std::any::Any;
use std::mem::MaybeUninit;
use crate::serialization::data::{DataInput, DataInputReadObject};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;

#[derive(Default)]
pub struct JavaClassSerializer;

impl Serializer<Box<String>> for JavaClassSerializer {
  fn id(&self) -> i32 {
    -24
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<String> {
    input.read_string().unwrap().into()
  }

  fn write(&self, output: &mut ObjectDataOutput, object: Box<String>) {
    //noop
  }
}