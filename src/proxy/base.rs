use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use crate::cluster::service::ClusterService;
use crate::connection::registry::ConnectionRegistry;
use crate::core::distributed_object::DistributedObject;
use crate::invocation::InvocationReturnValue;
use crate::invocation::service::InvocationService;
use crate::listener::service::ListenerService;
use crate::partition_service::PartitionService;
use crate::protocol::client_message::ClientMessage;
use crate::proxy::Proxy;
use crate::serialization::heap_data::HeapData;
use crate::serialization::schema::Schema;
use crate::serialization::serializable::Serializable;
use crate::serialization::serializer::Serializer;
use crate::serialization::service::SerializationServiceV1;
use crate::util::maybe_future::MaybeFuture;

#[derive(Clone)]
pub struct ProxyBase {
  pub name: String,
  pub service_name: String,
  pub partition_service: Arc<PartitionService>,
  pub invocation_service: Arc<InvocationService>,
  pub serialization_service: Arc<SerializationServiceV1>,
  pub connection_registry: Arc<ConnectionRegistry>,
  pub listener_service: Arc<ListenerService>,
  pub cluster_service: Arc<ClusterService>,
}

impl ProxyBase {
  pub fn new(
    name: String,
    service_name: String,
    connection_registry: Arc<ConnectionRegistry>,
    partition_service: Arc<PartitionService>,
    invocation_service: Arc<InvocationService>,
    serialization_service: Arc<SerializationServiceV1>,
    listener_service: Arc<ListenerService>,
    cluster_service: Arc<ClusterService>,
  ) -> Self {
    ProxyBase {
      name,
      connection_registry,
      service_name,
      partition_service,
      invocation_service,
      serialization_service,
      listener_service,
      cluster_service,
    }
  }
  pub fn to_data<T: Serializable + 'static>(&self, object: Box<T>) -> HeapData {
    self.serialization_service.to_data(object)
  }

  pub async fn encode_invoke_on_key<R: InvocationReturnValue + Send + Sync + Clone>(&self, key_data: HeapData, encoder: Pin<Box<dyn Send + Sync + Fn(String) -> Pin<Box<dyn Send + Sync + Future<Output=ClientMessage>>>>>, decoder: Pin<Box<dyn Send + Sync + Fn(ClientMessage) -> Pin<Box<dyn Send + Sync + Future<Output=Box<Box<R>>>>>>>) -> R {
    let partition_id = self.partition_service.get_partition_id(key_data).await;
    self.encode_invoke_on_partition(partition_id, encoder, decoder).await
  }

  async fn encode_invoke_on_partition<R: InvocationReturnValue + Send + Sync + Clone>(&self, partition_id: i32, encoder: Pin<Box<dyn Send + Sync + Fn(String) -> Pin<Box<dyn Send + Sync + Future<Output=ClientMessage>>>>>, decoder: Pin<Box<dyn Send + Sync + Fn(ClientMessage) -> Pin<Box<dyn Send + Sync + Future<Output=Box<Box<R>>>>>>>) -> R {
    let client_message = encoder.call((self.name.clone(), )).await;
    self.invocation_service.invoke_on_partition(&self.connection_registry, client_message, partition_id, decoder).await
  }
}

pub trait ProxyBaseLogic: Sized + DistributedObject + HasProxyBase + Clone {
  fn destroy_locally(self);
  fn get_existing_proxy(name: String) -> Pin<Box<dyn Future<Output=Option<Box<MaybeFuture<Self>>>> + Send + Sync>>;
  fn register_proxy(name: String, proxy: MaybeFuture<Self>) -> Pin<Box<dyn Future<Output=()> + Send + Sync>>;
}

pub trait HasProxyBase {
  fn get_proxy_base(&self) -> &ProxyBase;
}

impl<T: Proxy> ProxyBaseLogic for T {
  fn destroy_locally(self) {
    todo!()
  }

  fn get_existing_proxy(name: String) -> Pin<Box<dyn Future<Output=Option<Box<MaybeFuture<Self>>>> + Send + Sync>> {
    Box::pin(async move {
      Self::get_proxies().read().await.get(&name).cloned()
    })
  }

  fn register_proxy(name: String, proxy: MaybeFuture<Self>) -> Pin<Box<dyn Future<Output=()> + Send + Sync>> {
    Box::pin(async move {
      Self::get_proxies().write().await.insert(name, Box::new(proxy));
    })
  }
}

#[async_trait_with_sync::async_trait]
impl<T: Proxy> DistributedObject for T {
  fn get_partition_key(&self) -> String {
    self.get_proxy_base().name.clone()
  }

  fn get_name(&self) -> String {
    self.get_proxy_base().name.clone()
  }

  fn get_service_name(&self) -> String {
    self.get_proxy_base().service_name.clone()
  }

  async fn destroy(self) {
    todo!()
  }
}
