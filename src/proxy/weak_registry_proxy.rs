use crate::nested_state::{NestedStateAdd, NestedStateGetBy, NestedStateGetOwnedBy};
use crate::proxy::map_proxy::MapProxy;
use crate::proxy::weak_map_proxy::WeakMapProxy;
use crate::serialization::serializable::Serializable;
use crate::util::observable_weak_arc::ObservableArc;
use std::future::Future;
use std::hash::Hash;
use std::ops::Deref;
use std::pin::Pin;

pub trait RegistryEntry: Clone + Send + Sync + 'static {
    type Key: Serializable + Clone + Send + Sync + Eq + PartialEq + Hash + ToString + 'static;

    fn get_key(&self) -> Self::Key;
}

pub struct WeakRegistryProxy<V: RegistryEntry + Serializable> {
    inner: WeakMapProxy<V::Key, V>,
}

impl<V: RegistryEntry + Serializable> WeakRegistryProxy<V> {
    pub fn new(inner: WeakMapProxy<V::Key, V>) -> Self {
        Self { inner }
    }
}

impl<V: RegistryEntry + Serializable> WeakRegistryProxy<V> {
    pub async fn add_weak(&self, value: impl Into<ObservableArc<V>>) {
        let value = value.into();
        self.inner.put_weak_value(value.get_key(), value).await;
    }
}
