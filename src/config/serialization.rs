use std::collections::HashMap;
use std::sync::Arc;
use derivative::Derivative;
use crate::serialization::serializable::{IdentifiedDataSerializable, IdentifiedDataSerializableSerialization, IdentifiedDataSerializableFactory, CustomSerializable};
use crate::serialization::serializer::Serializer;

#[derive(Clone, Derivative)]
#[derivative(Default)]
pub struct SerializationConfig {
  pub portable_version: i32,
  pub data_serializable_factories: Arc<HashMap<i32, Arc<IdentifiedDataSerializableFactory<Box<dyn IdentifiedDataSerializable>>>>>,
  pub custom_serializers: Arc<Vec<Box<dyn Serializer<Box<dyn CustomSerializable>>>>>,
  #[derivative(Default(value="true"))]
  pub is_big_endian: bool,
}