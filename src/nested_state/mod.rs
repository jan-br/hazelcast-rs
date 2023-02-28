use crate::client::HazelcastClient;
use crate::proxy::weak_registry_proxy::RegistryEntry;
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializable::Serializable;
use crate::serialization::serializer::Serializer;
use crate::serialization::service::SerializationServiceV1;
use crate::util::observable_weak_arc::ObservableArc;
use serde::Serialize;
use std::future::Future;
use std::marker::PhantomData;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

pub trait NestedStateAdd<T: Send + Sync + 'static> {
    fn add<'a>(
        &'a mut self,
        value: impl Into<T> + Send + Sync + 'a,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + Sync + 'a>>;
}

pub trait NestedStateRemove<T: Send + Sync + 'static> {
    fn remove<'a>(
        &'a mut self,
        value: impl Into<T> + Send + Sync + 'a,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + Sync + 'a>>;
}

pub trait NestedStateInsert<K: Send + Sync + 'static, V: Send + Sync + 'static> {
    fn insert<'a>(
        &'a mut self,
        key: impl Into<K> + Send + Sync + 'a,
        value: impl Into<V> + Send + Sync + 'a,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + Sync + 'a>>;
}

pub trait NestedStateGetBy<K: Send + Sync + 'static, V: Send + Sync + 'static> {
    fn get<'a>(
        &'a mut self,
        key: impl Into<K> + Send + Sync + 'a,
    ) -> Pin<Box<dyn Future<Output = V> + Send + Sync + 'a>>;
}

pub trait NestedStateGetOwnedBy<K: Send + Sync + 'static, V: Send + Sync + 'static> {
    fn get_owned<'a>(
        &'a mut self,
        key: impl Into<K> + Send + Sync + 'a,
    ) -> Pin<Box<dyn Future<Output = V> + Send + Sync + 'a>>;
}

pub struct NestedState<T> {
    inner: T,
    hazelcast_client: HazelcastClient,
}

impl<T> Deref for NestedState<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
