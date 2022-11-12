use crate::serialization::compact::default_compact_reader::DefaultCompactReader;
use crate::serialization::compact::default_compact_writer::DefaultCompactWriter;
use crate::serialization::generic_record::compact_generic_record::CompactGenericRecord;

pub trait FieldKindBasedOperations {
  fn write_field_from_record_to_writer(&self, writer: &mut DefaultCompactWriter, generic_record: &CompactGenericRecord, field_name: String);

  fn kind_size_in_bytes(&self) -> usize;

  fn read_from_reader(&self, reader: &mut DefaultCompactReader, field_name: String) -> CompactGenericRecord;

  fn validate_field(&self, field_name: String, value: &CompactGenericRecord, get_error_string_fn: Box<dyn Fn(String, String, &CompactGenericRecord) -> String>);
}