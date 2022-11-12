use crate::serialization::data::{DataOutput, DataInput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;

#[derive(Default)]
pub struct DoubleArraySerializer;

impl Serializer<Box<Vec<f64>>> for DoubleArraySerializer {
  fn id(&self) -> i32 {
    -19
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<Vec<f64>> {
    input.read_double_array().unwrap().into()
  }

  fn write(&self, output: &mut ObjectDataOutput, object: Box<Vec<f64>>) {
    output.write_double_array(Some(object.as_ref()));
  }
}