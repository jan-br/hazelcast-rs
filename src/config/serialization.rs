use std::collections::HashMap;
use std::sync::Arc;
use crate::serialization::serializable::{IdentifiedDataSerializable, IdentifiedDataSerializableSerialization, IdentifiedDataSerializableFactory, CustomSerializable};
use crate::serialization::serializer::Serializer;

#[derive(Clone, Default)]
pub struct SerializationConfig {
  pub portable_version: i32,
  pub data_serializable_factories: Arc<HashMap<i32, Arc<IdentifiedDataSerializableFactory<Box<dyn IdentifiedDataSerializable>>>>>,
  pub custom_serializers: Arc<Vec<Box<dyn Serializer<Box<dyn CustomSerializable>>>>>,
  pub is_big_endian: bool,
}