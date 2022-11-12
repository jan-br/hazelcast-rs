use crate::serialization::data::{DataOutput, DataInput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;

#[derive(Default)]
pub struct LongArraySerializer;

impl Serializer<Box<Vec<i64>>> for LongArraySerializer {
    fn id(&self) -> i32 {
        -17
    }

    fn read(&self, input: &mut ObjectDataInput) -> Box<Vec<i64>> {
        input.read_long_array().unwrap().into()
    }

    fn write(&self, output: &mut ObjectDataOutput, object: Box<Vec<i64>>) {
        output.write_long_array(Some(object.as_ref()));
    }
}