use std::any::Any;
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;

#[derive(Default)]
pub struct JsonSerializer;

impl Serializer<Box<dyn Any>> for JsonSerializer {
  fn id(&self) -> i32 {
    -130
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<dyn Any> {
    todo!()
  }

  fn write(&self, output: &mut ObjectDataOutput, object: Box<dyn Any>) {
    todo!()
  }
}