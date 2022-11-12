use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use crate::serialization::data::{DataInput, DataOutput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializable::{IdentifiedDataSerializable, IdentifiedDataSerializableSerialization, IdentifiedDataSerializableFactory, IdentifiedDataSerializableInfo};
use crate::serialization::serializer::Serializer;

pub struct IdentifiedDataSerializableSerializer {
  id: i32,
  factories: HashMap<i32, Arc<IdentifiedDataSerializableFactory<Box<dyn IdentifiedDataSerializable>>>>,
}

impl IdentifiedDataSerializableSerializer {
  pub fn new(factories: HashMap<i32, Arc<IdentifiedDataSerializableFactory<Box<dyn IdentifiedDataSerializable>>>>) -> Self {
    Self {
      factories,
      id: -2,
    }
  }
}

impl Serializer<Box<dyn IdentifiedDataSerializable>> for IdentifiedDataSerializableSerializer {
  fn id(&self) -> i32 {
    self.id
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<dyn IdentifiedDataSerializable> {
    let is_identified = input.read_boolean();
    if !is_identified {
      todo!()
    }
    let factory_id = input.read_int();
    let class_id = input.read_int();
    let factory_fn = self.factories.get(&factory_id);
    if factory_fn.is_none() {
      todo!()
    }
    let mut object: Box<dyn IdentifiedDataSerializable> = factory_fn.unwrap().call((class_id, ));
    object.read_data(input);
    object
  }

  fn write(&self, output: &mut ObjectDataOutput, mut object: Box<dyn IdentifiedDataSerializable>) {
    output.write_boolean(true);
    output.write_int(object.factory_id());
    output.write_int(object.class_id());
    object.write_data(output);
  }
}