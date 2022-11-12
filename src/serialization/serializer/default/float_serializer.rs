use crate::serialization::data::{DataOutput, DataInput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;

#[derive(Default)]
pub struct FloatSerializer;

impl Serializer<Box<f32>> for FloatSerializer {
  fn id(&self) -> i32 {
    -9
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<f32> {
    input.read_float().into()
  }

  fn write(&self, output: &mut ObjectDataOutput, object: Box<f32>) {
    output.write_float(*object);
  }
}