use chrono::{NaiveTime, Timelike};
use crate::serialization::data::{DataInput, DataOutput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;

#[derive(Default)]
pub struct LocalTimeSerializer;

impl Serializer<Box<NaiveTime>> for LocalTimeSerializer {
  fn id(&self) -> i32 {
    -52
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<NaiveTime> {
    let hour = input.read_byte();
    let minute = input.read_byte();
    let second = input.read_byte();
    let nanos = input.read_int();
    NaiveTime::from_hms_nano(hour as u32, minute as u32, second as u32, nanos as u32).into()
  }

  fn write(&self, output: &mut ObjectDataOutput, object: Box<NaiveTime>) {
    output.write_byte(object.hour() as u8);
    output.write_byte(object.minute() as u8);
    output.write_byte(object.second() as u8);
    output.write_int(object.nanosecond() as i32);
  }
}