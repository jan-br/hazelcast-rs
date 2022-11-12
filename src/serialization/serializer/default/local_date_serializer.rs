use chrono::{Datelike, NaiveDate};
use crate::serialization::data::{DataInput, DataOutput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;

#[derive(Default)]
pub struct LocalDateSerializer;

impl Serializer<Box<NaiveDate>> for LocalDateSerializer {
  fn id(&self) -> i32 {
    -51
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<NaiveDate> {
    let year = input.read_int();
    let month = input.read_byte();
    let date = input.read_byte();
    NaiveDate::from_ymd(year, month as u32, date as u32).into()
  }

  fn write(&self, output: &mut ObjectDataOutput, object: Box<NaiveDate>) {
    output.write_int(object.year());
    output.write_byte(object.month() as u8);
    output.write_byte(object.day() as u8);
  }
}