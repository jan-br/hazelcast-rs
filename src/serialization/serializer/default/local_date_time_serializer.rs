use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};
use crate::serialization::data::{DataInput, DataOutput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;

#[derive(Default)]
pub struct LocalDateTimeSerializer;

impl Serializer<Box<NaiveDateTime>> for LocalDateTimeSerializer {
  fn id(&self) -> i32 {
    -53
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<NaiveDateTime> {
    let year = input.read_int();
    let month = input.read_byte();
    let date = input.read_byte();

    let hour = input.read_byte();
    let minute = input.read_byte();
    let second = input.read_byte();
    let nanos = input.read_int();

    NaiveDate::from_ymd(year, month as u32, date as u32).and_hms_nano(hour as u32, minute as u32, second as u32, nanos as u32).into()
  }

  fn write(&self, output: &mut ObjectDataOutput, object: Box<NaiveDateTime>) {
    output.write_int(object.year());
    output.write_byte(object.month() as u8);
    output.write_byte(object.day() as u8);

    output.write_byte(object.hour() as u8);
    output.write_byte(object.minute() as u8);
    output.write_byte(object.second() as u8);
    output.write_int(object.nanosecond() as i32);
  }
}