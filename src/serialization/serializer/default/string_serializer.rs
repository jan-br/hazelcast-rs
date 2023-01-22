use std::mem::transmute;
use std::sync::Arc;
use crate::serialization::data::{DataInput, DataOutput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::default_serializer::IdentifiedDataSerializableSerializer;
use crate::serialization::serializable::{IdentifiedDataSerializable, Serializable, SomeSerializable};
use crate::serialization::serializer::Serializer;
use crate::serialization::service::SerializationServiceV1;

#[derive(Default)]
pub struct StringSerializer;

impl Serializable for Option<String> {
  fn get_serializer(&self, service: &SerializationServiceV1) -> Arc<dyn Serializer<Box<Self>>> {
    unsafe { transmute(service.registry.get(service.serializer_name_to_id.get(&"string".to_string()).unwrap()).unwrap().clone()) }
  }
}

impl Serializer<Box<Option<String>>> for StringSerializer {
  fn id(&self) -> i32 {
    -11
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<Option<String>> {
    Box::new(input.read_string())
  }

  fn write(&self, output: &mut ObjectDataOutput, mut object: Box<Option<String>>) {
    output.write_string((*object).as_ref());
  }
}

impl SomeSerializable for String {}