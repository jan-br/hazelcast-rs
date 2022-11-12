use crate::serialization::data::{DataOutput, DataInput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;

#[derive(Default)]
pub struct FloatArraySerializer;

impl Serializer<Box<Vec<f32>>> for FloatArraySerializer {
  fn id(&self) -> i32 {
    -18
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<Vec<f32>> {
    input.read_float_array().unwrap().into()
  }

  fn write(&self, output: &mut ObjectDataOutput, object: Box<Vec<f32>>) {
    output.write_float_array(Some(object.as_ref()));
  }
}