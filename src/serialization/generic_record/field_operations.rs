use crate::serialization::compact::default_compact_reader::DefaultCompactReader;
use crate::serialization::compact::default_compact_writer::DefaultCompactWriter;
use crate::serialization::generic_record::compact_generic_record::CompactGenericRecord;
use crate::serialization::generic_record::field_kind_based_operations::FieldKindBasedOperations;

pub struct FieldOperations {}

pub struct BooleanFieldKindBasedOperations;

impl FieldKindBasedOperations for BooleanFieldKindBasedOperations {
  fn write_field_from_record_to_writer(&self, writer: &mut DefaultCompactWriter, generic_record: &CompactGenericRecord, field_name: String) {
    todo!()
  }

  fn kind_size_in_bytes(&self) -> usize {
    todo!()
  }

  fn read_from_reader(&self, reader: &mut DefaultCompactReader, field_name: String) -> CompactGenericRecord {
    todo!()
  }

  fn validate_field(&self, field_name: String, value: &CompactGenericRecord, get_error_string_fn: Box<dyn Fn(String, String, &CompactGenericRecord) -> String>) {
    todo!()
  }
}

impl FieldOperations {
  pub const VARIABLE_SIZE: i32 = -1;
}