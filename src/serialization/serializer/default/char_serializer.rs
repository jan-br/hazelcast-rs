use crate::serialization::data::{DataOutput, DataInput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;

#[derive(Default)]
pub struct CharSerializer;

impl Serializer<Box<char>> for CharSerializer {
    fn id(&self) -> i32 {
        -5
    }

    fn read(&self, input: &mut ObjectDataInput) -> Box<char> {
        input.read_char().into()
    }

    fn write(&self, output: &mut ObjectDataOutput, object: Box<char>) {
        //noop
    }
}