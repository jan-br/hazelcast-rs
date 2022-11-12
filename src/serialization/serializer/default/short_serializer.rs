use crate::serialization::data::{DataInput, DataOutput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;

#[derive(Default)]
pub struct ShortSerializer;

impl Serializer<Box<i16>> for ShortSerializer {
  fn id(&self) -> i32 {
    -6
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<i16> {
    input.read_short().into()
  }

  fn write(&self, output: &mut ObjectDataOutput, object: Box<i16>) {
    output.write_short(*object);
  }
}