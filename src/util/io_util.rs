use chrono::{Datelike, DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use crate::core::big_decimal::BigDecimal;
use crate::serialization::data::{DataInput, DataOutput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::util::big_decimal_util::BigDecimalUtil;

pub struct IOUtil;

impl IOUtil {
  pub fn read_decimal(inp: &mut ObjectDataInput) -> BigDecimal {
    let buffer = inp.read_byte_array();
    let scale = inp.read_int();
    BigDecimal::new(BigDecimalUtil::buffer_to_big_int(&mut buffer.unwrap()), scale)
  }

  pub fn read_local_time(inp: &mut ObjectDataInput) -> NaiveTime {
    let hour = inp.read_byte();
    let minute = inp.read_byte();
    let second = inp.read_byte();
    let nano = inp.read_int();
    NaiveTime::from_hms_nano(hour as u32, minute as u32, second as u32, nano as u32)
  }

  pub fn read_local_date(inp: &mut ObjectDataInput) -> NaiveDate {
    let year = inp.read_int();
    let month = inp.read_byte();
    let day = inp.read_byte();
    NaiveDate::from_ymd(year, month as u32, day as u32)
  }

  pub fn read_local_date_time(inp: &mut ObjectDataInput) -> NaiveDateTime {
    let date = Self::read_local_date(inp);
    let time = Self::read_local_time(inp);
    NaiveDateTime::new(date, time)
  }

  pub fn read_offset_date_time(inp: &mut ObjectDataInput) -> DateTime<FixedOffset> {
    let local_date_time = Self::read_local_date_time(inp);
    let offset_seconds = inp.read_int();
    //todo: check if this is correct
    DateTime::from_utc(local_date_time, FixedOffset::east(offset_seconds))
  }

  pub fn write_decimal(out: &mut ObjectDataOutput, value: &BigDecimal) {
    out.write_byte_array(Some(&BigDecimalUtil::big_int_to_buffer(&value.unscaled_value)));
    out.write_int(value.scale);
  }

  pub fn write_local_time(out: &mut ObjectDataOutput, value: &NaiveTime) {
    out.write_byte(value.hour() as u8);
    out.write_byte(value.minute() as u8);
    out.write_byte(value.second() as u8);
    out.write_int(value.nanosecond() as i32);
  }

  pub fn write_local_date(out: &mut ObjectDataOutput, value: &NaiveDate) {
    out.write_int(value.year());
    out.write_byte(value.month() as u8);
    out.write_byte(value.day() as u8);
  }

  pub fn write_local_date_time(out: &mut ObjectDataOutput, value: &NaiveDateTime) {
    Self::write_local_date(out, &value.date());
    Self::write_local_time(out, &value.time());
  }

  pub fn write_offset_date_time(out: &mut ObjectDataOutput, value: &DateTime<FixedOffset>) {
    Self::write_local_date_time(out, &value.naive_local());
    //todo: check if this is correct
    out.write_int(value.offset().local_minus_utc());
  }
}