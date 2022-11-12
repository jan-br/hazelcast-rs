use crate::serialization::data::{DataInput, DataOutput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;

#[derive(Default)]
pub struct ByteSerializer;

impl Serializer<Box<u8>> for ByteSerializer {
    fn id(&self) -> i32 {
        -3
    }

    fn read(&self, input: &mut ObjectDataInput) -> Box<u8> {
        input.read_byte().into()
    }

    fn write(&self, output: &mut ObjectDataOutput, object: Box<u8>) {
        output.write_byte(*object);
    }
}