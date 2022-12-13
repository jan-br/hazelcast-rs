use std::mem::transmute;
use std::sync::Arc;
use crate::serialization::data::{DataInput, DataOutput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializable::Serializable;
use crate::serialization::serializer::Serializer;
use crate::serialization::service::SerializationServiceV1;

#[derive(Default)]
pub struct IntegerSerializer;

impl Serializable for i32 {
  fn get_serializer(&self, service: &SerializationServiceV1) -> Arc<dyn Serializer<Box<Self>>> {
    unsafe { transmute(service.registry.get(service.serializer_name_to_id.get(&"integer".to_string()).unwrap()).unwrap().clone()) }
  }
}

impl Serializer<Box<i32>> for IntegerSerializer {
  fn id(&self) -> i32 {
    -7
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<i32> {
    input.read_int().into()
  }

  fn write(&self, output: &mut ObjectDataOutput, object: Box<i32>) {
    output.write_int(*object);
  }
}