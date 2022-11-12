use crate::serialization::data::{DataInput, DataOutput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;

#[derive(Default)]
pub struct ByteArraySerializer;

impl Serializer<Box<Vec<u8>>> for ByteArraySerializer {
    fn id(&self) -> i32 {
        -12
    }

    fn read(&self, input: &mut ObjectDataInput) -> Box<Vec<u8>> {
        input.read_byte_array().unwrap().into()
    }

    fn write(&self, output: &mut ObjectDataOutput, object: Box<Vec<u8>>) {
        output.write_byte_array(Some(object.as_ref()));
    }
}