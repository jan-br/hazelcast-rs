use std::any::Any;
use std::collections::HashMap;
use crate::serialization::generic_record::field::FieldWithValue;
use crate::serialization::generic_record::generic_record::GenericRecord;
use crate::serialization::schema::Schema;

pub struct CompactGenericRecord {
  pub schema: Schema,
}

impl CompactGenericRecord {
  pub fn new(
    type_name: String,
    fields: HashMap<String, Box<FieldWithValue<dyn Any>>>,
    schema: Option<Schema>,
  ) -> Self {
    todo!()
  }
}

impl GenericRecord for CompactGenericRecord {}