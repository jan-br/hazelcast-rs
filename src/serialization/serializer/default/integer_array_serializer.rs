use crate::serialization::data::{DataOutput, DataInput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;

#[derive(Default)]
pub struct IntegerArraySerializer;

impl Serializer<Box<Vec<i32>>> for IntegerArraySerializer {
  fn id(&self) -> i32 {
    -16
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<Vec<i32>> {
    input.read_int_array().unwrap().into()
  }

  fn write(&self, output: &mut ObjectDataOutput, object: Box<Vec<i32>>) {
    output.write_int_array(Some(object.as_ref()));
  }
}