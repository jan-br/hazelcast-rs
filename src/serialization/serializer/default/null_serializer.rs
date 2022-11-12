use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;

#[derive(Default)]
pub struct NullSerializer;

impl NullSerializer {
  pub const NULL_TYPE_ID: i32 = 0;
}

impl Serializer<Box<()>> for NullSerializer {
  fn id(&self) -> i32 {
    Self::NULL_TYPE_ID
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<()> {
    ().into()
  }

  fn write(&self, output: &mut ObjectDataOutput, _: Box<()>) {
  }
}