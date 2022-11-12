use std::any::Any;
use std::sync::Arc;

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};
use crate::core::big_decimal::BigDecimal;
use crate::serialization::compact::compact_stream_serializer::CompactStreamSerializer;
use crate::serialization::compact::compact_writer::{CompactWriter, CompactWriterWriteArrayOfComponent, CompactWriterWriteCompact};
use crate::serialization::compact::offset_constants::OffsetConstants;
use crate::serialization::data::{DataOutput, PositionalOutput};
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::generic_record::compact_generic_record::CompactGenericRecord;
use crate::serialization::generic_record::field_descriptor::FieldDescriptor;
use crate::serialization::generic_record::field_kind::FieldKind;
use crate::serialization::schema::Schema;
use crate::util::bits_util::BitsUtil;
use crate::util::io_util::IOUtil;

pub struct DefaultCompactWriter<'a, 'b> {
  data_start_position: usize,
  fields_offsets: Option<Vec<i32>>,
  serializer: &'b CompactStreamSerializer,
  out: &'a mut ObjectDataOutput,
  schema: Arc<Schema>,
}

impl<'a, 'b> DefaultCompactWriter<'a, 'b> {
  pub fn new(serializer: &'b CompactStreamSerializer, out: &'a mut ObjectDataOutput, schema: Arc<Schema>) -> Self {
    let mut fields_offsets = None;
    let mut data_start_position: usize = 0;
    if schema.number_var_size_fields != 0 {
      fields_offsets = Some(vec![0; schema.number_var_size_fields as usize]);
      data_start_position = out.position() + BitsUtil::INT_SIZE_IN_BYTES as usize;
      out.write_zero_bytes((schema.fixed_size_fields_length + BitsUtil::INT_SIZE_IN_BYTES as usize) as i32);
    } else {
      fields_offsets = None;
      data_start_position = out.position();
      out.write_zero_bytes(schema.fixed_size_fields_length as i32);
    }
    DefaultCompactWriter {
      data_start_position,
      fields_offsets,
      serializer,
      out,
      schema,
    }
  }
}

impl<'a, 'b, T: Any> CompactWriterWriteCompact<T> for DefaultCompactWriter<'a, 'b> {
  fn write_compact(&mut self, field_name: String, value: Option<&T>) {
    self.write_variable_size_field(field_name.clone(), FieldKind::Compact, value, |this, value| this.serializer.write_object(this.out, value));
  }
}

impl<'a, 'b, T: Any> CompactWriterWriteArrayOfComponent<T> for DefaultCompactWriter<'a, 'b> {
  fn write_array_of_compact(&mut self, field_name: String, value: Option<&Vec<Option<&T>>>) {
    self.write_array_of_variable_sizes(field_name.clone(), FieldKind::ArrayOfCompact, value, |this, value| this.serializer.write_object(this.out, *value));
  }
}

impl<'a, 'b> CompactWriter for DefaultCompactWriter<'a, 'b> {
  fn write_boolean(&mut self, field_name: String, value: bool) {
    let field_definition = self.check_field_definition(field_name.clone(), FieldKind::Boolean);
    let offset_in_bytes = field_definition.offset;
    let offset_in_bits = field_definition.bit_offset;
    let write_offset = offset_in_bytes + self.data_start_position as isize;
    self.out.pwrite_boolean_bit(write_offset as usize, offset_in_bits as usize, value);
  }
  fn write_int8(&mut self, field_name: String, value: i8) {
    let position = self.get_fixed_size_field_position(field_name.clone(), FieldKind::Int8);
    self.out.pwrite_int8(position as usize, value);
  }
  fn write_int16(&mut self, field_name: String, value: i16) {
    let position = self.get_fixed_size_field_position(field_name.clone(), FieldKind::Int16);
    self.out.pwrite_short(position as usize, value);
  }
  fn write_int32(&mut self, field_name: String, value: i32) {
    let position = self.get_fixed_size_field_position(field_name.clone(), FieldKind::Int32);
    self.out.pwrite_int(position as usize, value);
  }
  fn write_int64(&mut self, field_name: String, value: i64) {
    let position = self.get_fixed_size_field_position(field_name.clone(), FieldKind::Int64);
    self.out.pwrite_long(position as usize, value);
  }
  fn write_float32(&mut self, field_name: String, value: f32) {
    let position = self.get_fixed_size_field_position(field_name.clone(), FieldKind::Float32);
    self.out.pwrite_float(position as usize, value);
  }
  fn write_float64(&mut self, field_name: String, value: f64) {
    let position = self.get_fixed_size_field_position(field_name.clone(), FieldKind::Float64);
    self.out.pwrite_double(position as usize, value);
  }
  fn write_string(&mut self, field_name: String, value: String) {
    self.write_variable_size_field(field_name.clone(), FieldKind::String, Some(&value), |this, value| this.out.write_string(Some(value)));
  }
  fn write_decimal(&mut self, field_name: String, value: Option<&BigDecimal>) {
    self.write_variable_size_field(field_name.clone(), FieldKind::Decimal, value, |this, value| IOUtil::write_decimal(&mut this.out, value));
  }
  fn write_time(&mut self, field_name: String, value: Option<&NaiveTime>) {
    self.write_variable_size_field(field_name.clone(), FieldKind::Time, value, |this, value| IOUtil::write_local_time(&mut this.out, value));
  }
  fn write_date(&mut self, field_name: String, value: Option<&NaiveDate>) {
    self.write_variable_size_field(field_name.clone(), FieldKind::Date, value, |this, value| IOUtil::write_local_date(&mut this.out, value));
  }
  fn write_timestamp(&mut self, field_name: String, value: Option<&NaiveDateTime>) {
    self.write_variable_size_field(field_name.clone(), FieldKind::Timestamp, value, |this, value| IOUtil::write_local_date_time(&mut this.out, value));
  }
  fn write_timestamp_with_timezone(&mut self, field_name: String, value: Option<&DateTime<FixedOffset>>) {
    self.write_variable_size_field(field_name.clone(), FieldKind::TimestampWithTimezone, value, |this, value| IOUtil::write_offset_date_time(&mut this.out, value));
  }
  fn write_array_of_boolean(&mut self, field_name: String, value: Option<&Vec<bool>>) {
    self.write_variable_size_field(field_name.clone(), FieldKind::ArrayOfBoolean, value, |this, value| DefaultCompactWriter::write_boolean_bits(&mut this.out, value));
  }
  fn write_array_of_int8(&mut self, field_name: String, value: Option<&Vec<u8>>) {
    self.write_variable_size_field(field_name.clone(), FieldKind::ArrayOfInt8, value, |this, value| this.out.write_byte_array(Some(value)));
  }
  fn write_array_of_int16(&mut self, field_name: String, value: Option<&Vec<i16>>) {
    self.write_variable_size_field(field_name.clone(), FieldKind::ArrayOfInt16, value, |this, value| this.out.write_short_array(Some(value)));
  }
  fn write_array_of_int32(&mut self, field_name: String, value: Option<&Vec<i32>>) {
    self.write_variable_size_field(field_name.clone(), FieldKind::ArrayOfInt32, value, |this, value| this.out.write_int_array(Some(value)));
  }
  fn write_array_of_int64(&mut self, field_name: String, value: Option<&Vec<i64>>) {
    self.write_variable_size_field(field_name.clone(), FieldKind::ArrayOfInt64, value, |this, value| this.out.write_long_array(Some(value)));
  }
  fn write_array_of_float32(&mut self, field_name: String, value: Option<&Vec<f32>>) {
    self.write_variable_size_field(field_name.clone(), FieldKind::ArrayOfFloat32, value, |this, value| this.out.write_float_array(Some(value)));
  }
  fn write_array_of_float64(&mut self, field_name: String, value: Option<&Vec<f64>>) {
    self.write_variable_size_field(field_name.clone(), FieldKind::ArrayOfFloat64, value, |this, value| this.out.write_double_array(Some(value)));
  }
  fn write_array_of_string(&mut self, field_name: String, value: Option<&Vec<Option<String>>>) {
    self.write_array_of_variable_sizes(field_name.clone(), FieldKind::ArrayOfString, value, |this, value| this.out.write_string(Some(value)));
  }
  fn write_array_of_decimal(&mut self, field_name: String, value: Option<&Vec<Option<BigDecimal>>>) {
    self.write_array_of_variable_sizes(field_name.clone(), FieldKind::ArrayOfDecimal, value, |this, value| IOUtil::write_decimal(&mut this.out, value));
  }
  fn write_array_of_time(&mut self, field_name: String, value: Option<&Vec<Option<NaiveTime>>>) {
    self.write_array_of_variable_sizes(field_name.clone(), FieldKind::ArrayOfTime, value, |this, value| IOUtil::write_local_time(&mut this.out, value));
  }
  fn write_array_of_date(&mut self, field_name: String, value: Option<&Vec<Option<NaiveDate>>>) {
    self.write_array_of_variable_sizes(field_name.clone(), FieldKind::ArrayOfDate, value, |this, value| IOUtil::write_local_date(&mut this.out, value));
  }
  fn write_array_of_timestamp(&mut self, field_name: String, value: Option<&Vec<Option<NaiveDateTime>>>) {
    self.write_array_of_variable_sizes(field_name.clone(), FieldKind::ArrayOfTimestamp, value, |this, value| IOUtil::write_local_date_time(&mut this.out, value));
  }
  fn write_array_of_timestamp_with_timezone(&mut self, field_name: String, value: Option<&Vec<Option<DateTime<FixedOffset>>>>) {
    self.write_array_of_variable_sizes(field_name.clone(), FieldKind::ArrayOfTimestampWithTimezone, value, |this, value| IOUtil::write_offset_date_time(&mut this.out, value));
  }
  fn write_nullable_boolean(&mut self, field_name: String, value: Option<&bool>) {
    self.write_variable_size_field(field_name.clone(), FieldKind::NullableBoolean, value, |this, value| this.out.write_boolean(*value));
  }
  fn write_nullable_int8(&mut self, field_name: String, value: Option<&i8>) {
    self.write_variable_size_field(field_name.clone(), FieldKind::NullableInt8, value, |this, value| this.out.write_int8(*value));
  }
  fn write_nullable_int16(&mut self, field_name: String, value: Option<&i16>) {
    self.write_variable_size_field(field_name.clone(), FieldKind::NullableInt16, value, |this, value| this.out.write_short(*value));
  }
  fn write_nullable_int32(&mut self, field_name: String, value: Option<&i32>) {
    self.write_variable_size_field(field_name.clone(), FieldKind::NullableInt32, value, |this, value| this.out.write_int(*value));
  }
  fn write_nullable_int64(&mut self, field_name: String, value: Option<&i64>) {
    self.write_variable_size_field(field_name.clone(), FieldKind::NullableInt64, value, |this, value| this.out.write_long(*value));
  }
  fn write_nullable_float32(&mut self, field_name: String, value: Option<&f32>) {
    self.write_variable_size_field(field_name.clone(), FieldKind::NullableFloat32, value, |this, value| this.out.write_float(*value));
  }
  fn write_nullable_float64(&mut self, field_name: String, value: Option<&f64>) {
    self.write_variable_size_field(field_name.clone(), FieldKind::NullableFloat64, value, |this, value| this.out.write_double(*value));
  }
  fn write_array_of_nullable_boolean(&mut self, field_name: String, value: Option<&Vec<Option<bool>>>) {
    self.write_array_of_variable_sizes(field_name.clone(), FieldKind::ArrayOfNullableBoolean, value, |this, value| this.out.write_boolean(*value));
  }
  fn write_array_of_nullable_int8(&mut self, field_name: String, value: Option<&Vec<Option<i8>>>) {
    self.write_array_of_variable_sizes(field_name.clone(), FieldKind::ArrayOfNullableInt8, value, |this, value| this.out.write_int8(*value));
  }
  fn write_array_of_nullable_int16(&mut self, field_name: String, value: Option<&Vec<Option<i16>>>) {
    self.write_array_of_variable_sizes(field_name.clone(), FieldKind::ArrayOfNullableInt16, value, |this, value| this.out.write_short(*value));
  }
  fn write_array_of_nullable_int32(&mut self, field_name: String, value: Option<&Vec<Option<i32>>>) {
    self.write_array_of_variable_sizes(field_name.clone(), FieldKind::ArrayOfNullableInt32, value, |this, value| this.out.write_int(*value));
  }
  fn write_array_of_nullable_int64(&mut self, field_name: String, value: Option<&Vec<Option<i64>>>) {
    self.write_array_of_variable_sizes(field_name.clone(), FieldKind::ArrayOfNullableInt64, value, |this, value| this.out.write_long(*value));
  }
  fn write_array_of_nullable_float32(&mut self, field_name: String, value: Option<&Vec<Option<f32>>>) {
    self.write_array_of_variable_sizes(field_name.clone(), FieldKind::ArrayOfNullableFloat32, value, |this, value| this.out.write_float(*value));
  }
  fn write_array_of_nullable_float64(&mut self, field_name: String, value: Option<&Vec<Option<f64>>>) {
    self.write_array_of_variable_sizes(field_name.clone(), FieldKind::ArrayOfNullableFloat64, value, |this, value| this.out.write_double(*value));
  }
}

impl<'a, 'b> DefaultCompactWriter<'a, 'b> {
  pub fn write_generic_record(&mut self, field_name: String, value: &CompactGenericRecord) {
    self.write_variable_size_field(field_name.clone(), FieldKind::Compact, Some(value), |this, value| this.serializer.write_generic_record(&mut this.out, value));
  }
  pub fn write_array_of_generic_record(&mut self, field_name: String, value: Option<&Vec<Option<CompactGenericRecord>>>) {
    self.write_array_of_variable_sizes(field_name.clone(), FieldKind::ArrayOfCompact, value, |this, value| this.serializer.write_generic_record(&mut this.out, value));
  }
  pub fn end(&mut self) {
    if self.schema.number_var_size_fields == 0 {
      return;
    }
    let position = self.out.position().clone();
    let data_length = position - self.data_start_position.clone();
    self.write_offsets(data_length as i32, &self.fields_offsets.clone().unwrap());

    self.out.pwrite_int(self.data_start_position - BitsUtil::INT_SIZE_IN_BYTES as usize, data_length as i32);
  }
  pub fn write_offsets(&mut self, data_length: i32, offsets: &Vec<i32>) {
    if data_length < OffsetConstants::BYTE_OFFSET_READER_RANGE {
      for offset in offsets {
        self.out.write_byte(*offset as u8);
      }
    } else if data_length > OffsetConstants::SHORT_OFFSET_READER_RANGE {
      for offset in offsets {
        self.out.write_short(*offset as i16);
      }
    } else {
      for offset in offsets {
        self.out.write_int(*offset as i32);
      }
    }
  }
  pub fn write_variable_size_field<T: Any>(
    &mut self,
    field_name: String,
    field_kind: FieldKind,
    object: Option<&T>,
    write_fn: impl Fn(&mut Self, &T),
  ) {
    if object.is_none() {
      self.set_positions_as_null(field_name, field_kind);
    } else {
      self.set_position(field_name, field_kind);
      write_fn(self, object.unwrap());
    }
  }
  pub fn set_positions_as_null(&mut self, field_name: String, field_kind: FieldKind) {
    let field = self.check_field_definition(field_name, field_kind);
    let index = field.index;
    self.fields_offsets.as_mut().unwrap()[index as usize] = -1;
  }
  pub fn write_array_of_variable_sizes<T>(
    &mut self,
    field_name: String,
    field_kind: FieldKind,
    values: Option<&Vec<Option<T>>>,
    write_fn: impl Fn(&mut Self, &T),
  ) {
    if values.is_none() {
      self.set_position_as_null(field_name, field_kind);
      return;
    }
    self.set_position(field_name, field_kind);
    let data_length_offset = self.out.position();
    self.out.write_zero_bytes(BitsUtil::INT_SIZE_IN_BYTES);
    let item_count = values.unwrap().len();
    self.out.write_int(item_count as i32);

    let offset = self.out.position();
    let mut offsets = vec![0_i32; item_count];
    for i in 0..item_count {
      if values.unwrap()[i].is_some() {
        offsets[i] = (self.out.position() - offset) as i32;
        write_fn(self, &values.unwrap()[i].as_ref().unwrap());
      } else {
        offsets[i] = OffsetConstants::NULL_OFFSET;
      }
    }

    let data_length = self.out.position() - offset;
    self.out.pwrite_int(data_length_offset, data_length as i32);
    self.write_offsets(data_length as i32, &offsets);
  }
  pub fn set_position_as_null(&mut self, field_name: String, field_kind: FieldKind) {
    let field = self.check_field_definition(field_name, field_kind);
    let index = field.index;
    self.fields_offsets.as_mut().unwrap()[index as usize] = -1;
  }
  pub fn set_position(&mut self, field_name: String, field_kind: FieldKind) {
    let field = self.check_field_definition(field_name, field_kind);
    let position = self.out.position();
    let field_position = position - self.data_start_position;
    let index = field.index;
    self.fields_offsets.as_mut().unwrap()[index as usize] = field_position as i32;
  }
  pub fn check_field_definition(&self, field_name: String, field_kind: FieldKind) -> FieldDescriptor {
    let field = self.schema.field_definition_map.get(&field_name);
    if field.is_none() {
      panic!("Field {} is not defined in schema", field_name);
    }
    if field.unwrap().kind != field_kind as i32 {
      panic!("Field {} is has wrong type", field_name);
    }
    field.unwrap().clone()
  }
  pub fn get_fixed_size_field_position(
    &self,
    field_name: String,
    field_kind: FieldKind,
  ) -> i32 {
    let field_definition = self.check_field_definition(field_name, field_kind);
    (field_definition.offset as usize + self.data_start_position) as i32
  }
  pub fn write_boolean_bits(out: &mut ObjectDataOutput, booleans: &Vec<bool>) {
    let length = booleans.len();
    out.write_int(length as i32);
    let mut position = out.position();
    if length > 0 {
      let mut index = 0;
      out.write_zero_bytes(1);
      for boolean in booleans {
        if index == BitsUtil::BITS_IN_A_BYTE {
          index = 0;
          out.write_zero_bytes(1);
          position += 1;
        }
        out.pwrite_boolean_bit(position, index as usize, *boolean);
        index += 1;
      }
    }
  }
}