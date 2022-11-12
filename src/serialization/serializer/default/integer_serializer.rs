use crate::serialization::data::{DataInput, DataOutput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;

#[derive(Default)]
pub struct IntegerSerializer;

impl Serializer<Box<i32>> for IntegerSerializer {
  fn id(&self) -> i32 {
    -7
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<i32> {
    input.read_int().into()
  }

  fn write(&self, output: &mut ObjectDataOutput, object: Box<i32>) {
    output.write_int(*object);
  }
}