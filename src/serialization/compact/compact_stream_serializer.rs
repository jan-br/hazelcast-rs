use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::mem::{MaybeUninit, transmute};
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::serialization::compact::compact_reader::CompactReader;
use crate::serialization::compact::compact_serializer::CompactSerializer;
use crate::serialization::compact::default_compact_reader::DefaultCompactReader;
use crate::serialization::compact::default_compact_writer::DefaultCompactWriter;
use crate::serialization::compact::schema_writer::SchemaWriter;
use crate::serialization::data::{DataInput, DataOutput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::generic_record::compact_generic_record::CompactGenericRecord;
use crate::serialization::generic_record::generic_record::GenericRecord;
use crate::serialization::schema::Schema;
use crate::serialization::schema_service::SchemaService;
use crate::serialization::serializer::Serializer;

pub struct CompactStreamSerializer {
  pub id: i32,
  pub class_to_serializer_map: HashMap<TypeId, Box<dyn CompactSerializer<Box<dyn Any>>>>,
  pub schema_service: Arc<SchemaService>,
  pub type_to_schema_map: HashMap<TypeId, Arc<Schema>>,
  pub type_name_to_serializers_map: HashMap<String, Box<dyn CompactSerializer<Box<dyn Any>>>>,
}

impl CompactStreamSerializer {
  pub fn new(schema_service: Arc<SchemaService>) -> Self {
    CompactStreamSerializer {
      id: -55,
      schema_service,
      class_to_serializer_map: HashMap::new(),
      type_to_schema_map: HashMap::new(),
      type_name_to_serializers_map: HashMap::new(),
    }
  }

  pub fn is_registered_as_compact(&self, type_id: &TypeId) -> bool {
    self.class_to_serializer_map.contains_key(type_id)
  }

  pub fn write_object<T: Any>(&self, output: &mut ObjectDataOutput, obj: &T) {
    let type_id = TypeId::of::<T>();
    if let Some(compact_serializer) = self.class_to_serializer_map.get(&type_id) {
      let compact_serializer: &Box<dyn CompactSerializer<Box<&T>>> = unsafe { transmute(compact_serializer) };
      let schema = {
        let mut schema = self.type_to_schema_map.get(&type_id).map(|schema| schema.clone());
        if schema.is_none() {
          let mut writer = SchemaWriter::new(compact_serializer.get_type_name());
          compact_serializer.write(&mut writer, &Box::new(obj));
          schema = Some(Arc::new(writer.build()));
          self.validate_schema_replicated_to_cluster(schema.as_ref().unwrap(), &(*obj).type_id());
        }
        schema.unwrap().clone()
      };
      self.write_schema_and_object(compact_serializer, output, schema, obj);
    } else {
      todo!()
    }
  }

  pub fn write_schema_and_object<'a, T: 'static>(&self, compact_serializer: &Box<dyn CompactSerializer<Box<&'a T>>>, mut output: &mut ObjectDataOutput, schema: Arc<Schema>, obj: &'a T) {
    self.write_schema(&mut output, schema.clone());
    let mut writer = DefaultCompactWriter::new(self, output, schema);
    compact_serializer.write(&mut writer, &Box::new(obj));
    writer.end();
  }

  pub fn write_schema(&self, output: &mut ObjectDataOutput, schema: Arc<Schema>) {
    output.write_long(schema.schema_id);
  }

  pub fn validate_schema_replicated_to_cluster(&self, schema: &Schema, type_id: &TypeId) {
    todo!()
  }

  pub fn write_generic_record(&self, out: &mut ObjectDataOutput, generic_record: &dyn GenericRecord) {
    todo!()
  }

  pub fn get_or_read_schema(&self, input: &mut ObjectDataInput) -> Arc<Schema> {
    let schema_id = input.read_long();
    let schema = self.schema_service.get(schema_id);
    if schema.is_none() {
      todo!()
    }
    schema.unwrap()
  }
}

impl Serializer<Box<dyn GenericRecord>> for CompactStreamSerializer {
  fn id(&self) -> i32 {
    -55
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<dyn GenericRecord> {
    let schema = self.get_or_read_schema(input);
    let serializer = self.type_name_to_serializers_map.get(&schema.type_name);
    if serializer.is_none() {
      todo!()
    }
    let serializer = serializer.unwrap();

    let mut reader = DefaultCompactReader::new(&self, input, schema);

    unsafe { transmute(serializer.read(&mut reader)) }
  }

  fn write(&self, output: &mut ObjectDataOutput, object: Box<dyn GenericRecord>) {
    self.write_generic_record(output, object.as_ref());
  }
}
