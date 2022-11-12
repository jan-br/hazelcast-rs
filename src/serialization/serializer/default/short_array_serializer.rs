use crate::serialization::data::{DataInput, DataOutput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;

#[derive(Default)]
pub struct ShortArraySerializer;

impl Serializer<Box<Vec<i16>>> for ShortArraySerializer {
  fn id(&self) -> i32 {
    -15
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<Vec<i16>> {
    input.read_short_array().unwrap().into()
  }

  fn write(&self, output: &mut ObjectDataOutput, object: Box<Vec<i16>>) {
    output.write_short_array(Some(object.as_ref()));
  }
}