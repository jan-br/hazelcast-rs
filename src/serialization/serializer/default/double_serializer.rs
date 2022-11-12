use crate::serialization::data::{DataOutput, DataInput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;

#[derive(Default)]
pub struct DoubleSerializer;

impl Serializer<Box<f64>> for DoubleSerializer {
  fn id(&self) -> i32 {
    -10
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<f64> {
    input.read_double().into()
  }

  fn write(&self, output: &mut ObjectDataOutput, object: Box<f64>) {
    output.write_double(*object);
  }
}