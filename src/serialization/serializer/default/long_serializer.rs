use crate::serialization::data::{DataInput, DataOutput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;

#[derive(Default)]
pub struct LongSerializer;

impl Serializer<Box<i64>> for LongSerializer {
  fn id(&self) -> i32 {
    -8
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<i64> {
    input.read_long().into()
  }

  fn write(&self, output: &mut ObjectDataOutput, object: Box<i64>) {
    output.write_long(*object);
  }
}