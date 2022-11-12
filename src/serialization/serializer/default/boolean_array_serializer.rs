use crate::serialization::data::{DataInput, DataOutput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;

#[derive(Default)]
pub struct BooleanArraySerializer;

impl Serializer<Box<Vec<bool>>> for BooleanArraySerializer {
  fn id(&self) -> i32 {
    -13
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<Vec<bool>> {
    input.read_boolean_array().unwrap().into()
  }

  fn write(&self, output: &mut ObjectDataOutput, object: Box<Vec<bool>>) {
    output.write_boolean_array(Some(object.as_ref()));
  }
}