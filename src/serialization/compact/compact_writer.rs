use std::any::Any;

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};
use crate::core::big_decimal::BigDecimal;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::generic_record::compact_generic_record::CompactGenericRecord;
use crate::serialization::generic_record::field_descriptor::FieldDescriptor;
use crate::serialization::generic_record::field_kind::FieldKind;

pub trait CompactWriterWriteCompact<T>: CompactWriter {
  fn write_compact(&mut self, field_name: String, value: Option<&T>);
}

pub trait CompactWriterWriteArrayOfComponent<T>: CompactWriter {
  fn write_array_of_compact(&mut self, field_name: String, value: Option<&Vec<Option<&T>>>);
}

pub trait CompactWriter {
  fn write_boolean(&mut self, field_name: String, value: bool);
  fn write_int8(&mut self, field_name: String, value: i8);
  fn write_int16(&mut self, field_name: String, value: i16);
  fn write_int32(&mut self, field_name: String, value: i32);
  fn write_int64(&mut self, field_name: String, value: i64);
  fn write_float32(&mut self, field_name: String, value: f32);
  fn write_float64(&mut self, field_name: String, value: f64);
  fn write_string(&mut self, field_name: String, value: String);
  fn write_decimal(&mut self, field_name: String, value: Option<&BigDecimal>);
  fn write_time(&mut self, field_name: String, value: Option<&NaiveTime>);
  fn write_date(&mut self, field_name: String, value: Option<&NaiveDate>);
  fn write_timestamp(&mut self, field_name: String, value: Option<&NaiveDateTime>);
  fn write_timestamp_with_timezone(&mut self, field_name: String, value: Option<&DateTime<FixedOffset>>);
  fn write_array_of_boolean(&mut self, field_name: String, value: Option<&Vec<bool>>);
  fn write_array_of_int8(&mut self, field_name: String, value: Option<&Vec<u8>>);
  fn write_array_of_int16(&mut self, field_name: String, value: Option<&Vec<i16>>);
  fn write_array_of_int32(&mut self, field_name: String, value: Option<&Vec<i32>>);
  fn write_array_of_int64(&mut self, field_name: String, value: Option<&Vec<i64>>);
  fn write_array_of_float32(&mut self, field_name: String, value: Option<&Vec<f32>>);
  fn write_array_of_float64(&mut self, field_name: String, value: Option<&Vec<f64>>);
  fn write_array_of_string(&mut self, field_name: String, value: Option<&Vec<Option<String>>>);
  fn write_array_of_decimal(&mut self, field_name: String, value: Option<&Vec<Option<BigDecimal>>>);
  fn write_array_of_time(&mut self, field_name: String, value: Option<&Vec<Option<NaiveTime>>>);
  fn write_array_of_date(&mut self, field_name: String, value: Option<&Vec<Option<NaiveDate>>>);
  fn write_array_of_timestamp(&mut self, field_name: String, value: Option<&Vec<Option<NaiveDateTime>>>);
  fn write_array_of_timestamp_with_timezone(&mut self, field_name: String, value: Option<&Vec<Option<DateTime<FixedOffset>>>>);
  fn write_nullable_boolean(&mut self, field_name: String, value: Option<&bool>);
  fn write_nullable_int8(&mut self, field_name: String, value: Option<&i8>);
  fn write_nullable_int16(&mut self, field_name: String, value: Option<&i16>);
  fn write_nullable_int32(&mut self, field_name: String, value: Option<&i32>);
  fn write_nullable_int64(&mut self, field_name: String, value: Option<&i64>);
  fn write_nullable_float32(&mut self, field_name: String, value: Option<&f32>);
  fn write_nullable_float64(&mut self, field_name: String, value: Option<&f64>);
  fn write_array_of_nullable_boolean(&mut self, field_name: String, value: Option<&Vec<Option<bool>>>);
  fn write_array_of_nullable_int8(&mut self, field_name: String, value: Option<&Vec<Option<i8>>>);
  fn write_array_of_nullable_int16(&mut self, field_name: String, value: Option<&Vec<Option<i16>>>);
  fn write_array_of_nullable_int32(&mut self, field_name: String, value: Option<&Vec<Option<i32>>>);
  fn write_array_of_nullable_int64(&mut self, field_name: String, value: Option<&Vec<Option<i64>>>);
  fn write_array_of_nullable_float32(&mut self, field_name: String, value: Option<&Vec<Option<f32>>>);
  fn write_array_of_nullable_float64(&mut self, field_name: String, value: Option<&Vec<Option<f64>>>);
}