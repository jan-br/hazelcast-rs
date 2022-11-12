use chrono::{Datelike, DateTime, FixedOffset, NaiveDate, Timelike};
use crate::serialization::data::{DataInput, DataOutput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;

#[derive(Default)]
pub struct OffsetDateTimeSerializer;

impl Serializer<Box<DateTime<FixedOffset>>> for OffsetDateTimeSerializer {
  fn id(&self) -> i32 {
    -54
  }
  fn read(&self, input: &mut ObjectDataInput) -> Box<DateTime<FixedOffset>> {
    let year = input.read_int();
    let month = input.read_byte();
    let date = input.read_byte();

    let hour = input.read_byte();
    let minute = input.read_byte();
    let second = input.read_byte();
    let nanos = input.read_int();

    let offset = input.read_int();

    //todo: check if this is correct
    DateTime::from_utc(NaiveDate::from_ymd(year, month as u32, date as u32).and_hms_nano(hour as u32, minute as u32, second as u32, nanos as u32), FixedOffset::east(offset)).into()
  }

  fn write(&self, output: &mut ObjectDataOutput, object: Box<DateTime<FixedOffset>>) {
    output.write_int(object.year());
    output.write_byte(object.month() as u8);
    output.write_byte(object.day() as u8);

    output.write_byte(object.hour() as u8);
    output.write_byte(object.minute() as u8);
    output.write_byte(object.second() as u8);
    output.write_int(object.nanosecond() as i32);

    //todo: check if this is correct
    output.write_int(object.offset().local_minus_utc() as i32);
  }
}