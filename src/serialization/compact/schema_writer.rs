use std::any::Any;
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};

use crate::core::big_decimal::BigDecimal;
use crate::serialization::compact::compact_writer::{CompactWriter, CompactWriterWriteArrayOfComponent, CompactWriterWriteCompact};
use crate::serialization::generic_record::field_descriptor::FieldDescriptor;
use crate::serialization::generic_record::field_kind::FieldKind;
use crate::serialization::schema::Schema;

pub struct SchemaWriter {
  pub type_name: String,
  pub fields: Vec<FieldDescriptor>,
}

impl SchemaWriter {
  pub fn new(type_name: String) -> Self {
    Self {
      type_name,
      fields: vec![],
    }
  }
}

impl<T> CompactWriterWriteCompact<T> for SchemaWriter {
  fn write_compact(&mut self, field_name: String, value: Option<&T>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::Compact));
  }
}

impl<T> CompactWriterWriteArrayOfComponent<T> for SchemaWriter {
  fn write_array_of_compact(&mut self, field_name: String, value: Option<&Vec<Option<&T>>>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::ArrayOfCompact));
  }
}

impl CompactWriter for SchemaWriter {
  fn write_boolean(&mut self, field_name: String, value: bool) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::Boolean));
  }

  fn write_int8(&mut self, field_name: String, value: i8) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::Int8));
  }

  fn write_int16(&mut self, field_name: String, value: i16) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::Int16));
  }

  fn write_int32(&mut self, field_name: String, value: i32) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::Int32));
  }

  fn write_int64(&mut self, field_name: String, value: i64) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::Int64));
  }

  fn write_float32(&mut self, field_name: String, value: f32) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::Float32));
  }

  fn write_float64(&mut self, field_name: String, value: f64) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::Float64));
  }

  fn write_string(&mut self, field_name: String, value: String) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::String));
  }

  fn write_decimal(&mut self, field_name: String, value: Option<&BigDecimal>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::Decimal));
  }

  fn write_time(&mut self, field_name: String, value: Option<&NaiveTime>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::Time));
  }

  fn write_date(&mut self, field_name: String, value: Option<&NaiveDate>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::Date));
  }

  fn write_timestamp(&mut self, field_name: String, value: Option<&NaiveDateTime>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::Timestamp));
  }

  fn write_timestamp_with_timezone(&mut self, field_name: String, value: Option<&DateTime<FixedOffset>>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::TimestampWithTimezone));
  }

  fn write_array_of_boolean(&mut self, field_name: String, value: Option<&Vec<bool>>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::ArrayOfBoolean));
  }

  fn write_array_of_int8(&mut self, field_name: String, value: Option<&Vec<u8>>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::ArrayOfInt8));
  }

  fn write_array_of_int16(&mut self, field_name: String, value: Option<&Vec<i16>>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::ArrayOfInt16));
  }

  fn write_array_of_int32(&mut self, field_name: String, value: Option<&Vec<i32>>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::ArrayOfInt32));
  }

  fn write_array_of_int64(&mut self, field_name: String, value: Option<&Vec<i64>>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::ArrayOfInt64));
  }

  fn write_array_of_float32(&mut self, field_name: String, value: Option<&Vec<f32>>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::ArrayOfFloat32));
  }

  fn write_array_of_float64(&mut self, field_name: String, value: Option<&Vec<f64>>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::ArrayOfFloat64));
  }

  fn write_array_of_string(&mut self, field_name: String, value: Option<&Vec<Option<String>>>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::ArrayOfString));
  }

  fn write_array_of_decimal(&mut self, field_name: String, value: Option<&Vec<Option<BigDecimal>>>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::ArrayOfDecimal));
  }

  fn write_array_of_time(&mut self, field_name: String, value: Option<&Vec<Option<NaiveTime>>>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::ArrayOfTime));
  }

  fn write_array_of_date(&mut self, field_name: String, value: Option<&Vec<Option<NaiveDate>>>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::ArrayOfDate));
  }

  fn write_array_of_timestamp(&mut self, field_name: String, value: Option<&Vec<Option<NaiveDateTime>>>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::ArrayOfTimestamp));
  }

  fn write_array_of_timestamp_with_timezone(&mut self, field_name: String, value: Option<&Vec<Option<DateTime<FixedOffset>>>>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::ArrayOfTimestampWithTimezone));
  }

  fn write_nullable_boolean(&mut self, field_name: String, value: Option<&bool>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::NullableBoolean));
  }

  fn write_nullable_int8(&mut self, field_name: String, value: Option<&i8>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::NullableInt8));
  }

  fn write_nullable_int16(&mut self, field_name: String, value: Option<&i16>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::NullableInt16));
  }

  fn write_nullable_int32(&mut self, field_name: String, value: Option<&i32>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::NullableInt32));
  }

  fn write_nullable_int64(&mut self, field_name: String, value: Option<&i64>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::NullableInt64));
  }

  fn write_nullable_float32(&mut self, field_name: String, value: Option<&f32>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::NullableFloat32));
  }

  fn write_nullable_float64(&mut self, field_name: String, value: Option<&f64>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::NullableFloat64));
  }

  fn write_array_of_nullable_boolean(&mut self, field_name: String, value: Option<&Vec<Option<bool>>>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::ArrayOfNullableBoolean));
  }

  fn write_array_of_nullable_int8(&mut self, field_name: String, value: Option<&Vec<Option<i8>>>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::ArrayOfNullableInt8));
  }

  fn write_array_of_nullable_int16(&mut self, field_name: String, value: Option<&Vec<Option<i16>>>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::ArrayOfNullableInt16));
  }

  fn write_array_of_nullable_int32(&mut self, field_name: String, value: Option<&Vec<Option<i32>>>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::ArrayOfNullableInt32));
  }

  fn write_array_of_nullable_int64(&mut self, field_name: String, value: Option<&Vec<Option<i64>>>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::ArrayOfNullableInt64));
  }

  fn write_array_of_nullable_float32(&mut self, field_name: String, value: Option<&Vec<Option<f32>>>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::ArrayOfNullableFloat32));
  }

  fn write_array_of_nullable_float64(&mut self, field_name: String, value: Option<&Vec<Option<f64>>>) {
    self.fields.push(FieldDescriptor::new(field_name, FieldKind::ArrayOfNullableFloat64));
  }
}

impl SchemaWriter {
  pub fn add_field(&mut self, field_descriptor: FieldDescriptor) {
    self.fields.push(field_descriptor);
  }

  pub fn build(mut self) -> Schema {
    self.fields.sort_by(|a, b| a.field_name.cmp(&b.field_name));
    Schema::new(self.type_name, self.fields)
  }
}