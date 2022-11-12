use crate::serialization::data::DataInput;
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;

#[derive(Default)]
pub struct CharArraySerializer;

impl Serializer<Box<Vec<char>>> for CharArraySerializer {
  fn id(&self) -> i32 {
    -14
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<Vec<char>> {
    input.read_char_array().unwrap().into()
  }

  fn write(&self, output: &mut ObjectDataOutput, object: Box<Vec<char>>) {
    //noop
  }
}