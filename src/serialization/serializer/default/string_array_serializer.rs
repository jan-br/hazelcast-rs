use crate::serialization::data::{DataOutput, DataInput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;

#[derive(Default)]
pub struct StringArraySerializer;

impl Serializer<Box<Vec<String>>> for StringArraySerializer {
    fn id(&self) -> i32 {
        -20
    }

    fn read(&self, input: &mut ObjectDataInput) -> Box<Vec<String>> {
        input.read_string_array().unwrap().into()
    }

    fn write(&self, output: &mut ObjectDataOutput, object: Box<Vec<String>>) {
        output.write_string_array(Some(object.as_ref()));
    }
}