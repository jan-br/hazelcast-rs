use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::Arc;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializable::Serializable;
use crate::serialization::serializer::Serializer;

pub struct JsonMappedSerializer<T: Serialize + Send + Sync + 'static> {
  inner: Arc<dyn Serializer<Box<String>>>,
  _phantom: PhantomData<T>,
}

impl<T: Serializable + Send + Sync + Serialize + DeserializeOwned + 'static> JsonMappedSerializer<T> {
  pub fn new(inner: Arc<dyn Serializer<Box<String>>>) -> Arc<Self> {
    Self {
      inner,
      _phantom: Default::default(),
    }.into()
  }
}

impl<T: Serializable + Send + Sync + Serialize + DeserializeOwned + 'static> Serializer<Box<T>> for JsonMappedSerializer<T> {
  fn id(&self) -> i32 {
    self.inner.id()
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<T> {
    serde_json::from_str(self.inner.read(input).as_str()).expect("Could not deserialize object")
  }

  fn write(&self, output: &mut ObjectDataOutput, object: Box<T>) {
    self.inner.write(output, Box::from(serde_json::to_string(object.deref()).expect("Could not serialize object")));
  }
}
