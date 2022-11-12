use uuid::Uuid;
use crate::serialization::data::{DataInput, DataOutput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;

#[derive(Default)]
pub struct UuidSerializer;

impl Serializer<Box<Uuid>> for UuidSerializer {
  fn id(&self) -> i32 {
    -21
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<Uuid> {
    let msb = input.read_long();
    let lsb = input.read_long();
    Uuid::from_u64_pair(msb as u64, lsb as u64).into()
  }

  fn write(&self, output: &mut ObjectDataOutput, object: Box<Uuid>) {
    let (msb, lsb) = object.as_u64_pair();
    output.write_long(msb as i64);
    output.write_long(lsb as i64);
  }
}
