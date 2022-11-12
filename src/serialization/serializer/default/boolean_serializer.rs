use crate::serialization::data::{DataInput, DataOutput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;

#[derive(Default)]
pub struct BooleanSerializer;

impl Serializer<Box<bool>> for BooleanSerializer {
  fn id(&self) -> i32 {
    -4
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<bool> {
    input.read_boolean().into()
  }

  fn write(&self, output: &mut ObjectDataOutput, object: Box<bool>) {
    output.write_boolean(*object);
  }
}