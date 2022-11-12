use std::any::Any;
use std::sync::Arc;
use crate::connection::address::Address;
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;
use crate::serialization::service::SerializationServiceV1;

pub type IdentifiedDataSerializableFactory<T: IdentifiedDataSerializableSerialization> = dyn Fn(i32) -> T + Send + Sync;

pub trait IdentifiedDataSerializableInfo {
  fn factory_id(&self) -> i32;
  fn class_id(&self) -> i32;
}

pub trait Serializable {
  fn get_serializer(&self, service: &SerializationServiceV1) -> Arc<dyn Serializer<Box<Self>>>;
}

pub trait CustomSerializable {

}

pub trait IdentifiedDataSerializable: Any + IdentifiedDataSerializableSerialization + IdentifiedDataSerializableInfo + Send + Sync {}

impl<T: Send + Sync + Any + IdentifiedDataSerializableSerialization + IdentifiedDataSerializableInfo> IdentifiedDataSerializable for T {}

pub trait IdentifiedDataSerializableSerialization {
  fn read_data(&mut self, input: &mut ObjectDataInput);
  fn write_data(&mut self, output: &mut ObjectDataOutput);
}

impl<T: IdentifiedDataSerializableInfo> IdentifiedDataSerializableInfo for &T {
  fn factory_id(&self) -> i32 {
    (*self).factory_id()
  }

  fn class_id(&self) -> i32 {
    (*self).class_id()
  }
}

impl<T: IdentifiedDataSerializableSerialization> IdentifiedDataSerializableSerialization for &mut T {
  fn read_data(&mut self, input: &mut ObjectDataInput) {
    (*self).read_data(input)
  }

  fn write_data(&mut self, output: &mut ObjectDataOutput) {
    (*self).write_data(output)
  }
}

//
// pub trait IdentifiedDataSerializableRef<T>: AsMut<T> + AsRef<T> {}
//
//
// impl<T: IdentifiedDataSerializable> IdentifiedDataSerializable for dyn IdentifiedDataSerializableRef<T> {
//   fn factory_id(&self) -> i32 {
//     self.as_ref().factory_id()
//   }
//
//   fn class_id(&self) -> i32 {
//     self.as_ref().class_id()
//   }
//
//   fn read_data(&mut self, input: &mut ObjectDataInput) {
//     self.as_mut().read_data(input);
//   }
//
//   fn write_data(&mut self, output: &mut ObjectDataOutput) {
//     self.as_mut().write_data(output);
//   }
// }