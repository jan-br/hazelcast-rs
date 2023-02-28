use std::ops::Deref;
use crate::proxy::map_proxy::MapProxy;
use crate::serialization::serializable::Serializable;

pub struct RegistryProxy<K: Serializable, V: Serializable> {
  converter: Box<dyn Fn(&V) -> K + Send + Sync>,
  inner: MapProxy<K, V>,
}

impl<K: Serializable + 'static, V: Serializable + 'static> RegistryProxy<K, V> {
  pub fn new(converter: impl Fn(&V) -> K + Send + Sync + 'static, inner: MapProxy<K, V>) -> Self {
    Self {
      converter: Box::new(converter),
      inner,
    }
  }
}

impl<K: Serializable + Send + Sync + Clone + 'static, V: Serializable + Send + Sync + Clone + 'static> RegistryProxy<K, V> {
  pub async fn add(&self, value: impl Into<V>) {
    let value = value.into();
    let key = self.converter.call((&value, ));
    self.inner.put(key, value).await;
  }
}