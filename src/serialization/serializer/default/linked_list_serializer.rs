use std::any::Any;
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializable::IdentifiedDataSerializable;
use crate::serialization::serializer::default::java_array_serializer::JavaArraySerializer;
use crate::serialization::serializer::Serializer;

#[derive(Default)]
pub struct LinkedListSerializer;

impl Serializer<Box<Vec<Box<dyn Any>>>> for LinkedListSerializer {
  fn id(&self) -> i32 {
    -30
  }

    fn read(&self, input: &mut ObjectDataInput) -> Box<Vec<Box<dyn Any>>> {
    JavaArraySerializer.read(input)
  }

    fn write(&self, output: &mut ObjectDataOutput, object: Box<Vec<Box<dyn Any>>>) {
    JavaArraySerializer.write(output, object)
  }
}