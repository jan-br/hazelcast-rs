use crate::codec::client_create_proxy_codec::ClientCreateProxyCodec;
use crate::connection::manager::ConnectionManager;
use crate::connection::registry::ConnectionRegistry;
use crate::core::distributed_object::DistributedObject;
use crate::invocation::service::InvocationService;
use crate::invocation::InvocationReturnValue;
use crate::protocol::client_message::ClientMessage;
use crate::proxy::base::ProxyBase;
use crate::proxy::map_proxy::MapProxy;
use crate::proxy::Proxy;
use crate::util::maybe_future::MaybeFuture;
use std::collections::HashMap;
use std::future::Future;
use std::mem::transmute;
use std::ops::Deref;
use std::pin::Pin;
use std::string::ToString;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use crate::partition_service::PartitionService;
use crate::serialization::service::SerializationServiceV1;

#[derive(Clone)]
pub struct ProxyManager {
  pub partition_service: Arc<PartitionService>,
  pub invocation_service: Arc<InvocationService>,
  pub connection_registry: Arc<ConnectionRegistry>,
  pub serialization_service: Arc<SerializationServiceV1>,
}

impl ProxyManager {
  pub const NAMESPACE_SEPERATOR: &'static str = "/";

  pub fn new(
    partition_service: Arc<PartitionService>,
    connection_registry: Arc<ConnectionRegistry>,
    invocation_service: Arc<InvocationService>,
    serialization_service: Arc<SerializationServiceV1>,
  ) -> Self {
    ProxyManager {
      partition_service,
      connection_registry,
      invocation_service,
      serialization_service,
    }
  }

  pub async fn get_or_create_proxy<T: Proxy + Sized + Send + Sync>(
    &self,
    name: impl ToString,
    create_at_server: bool,
  ) -> T {
    let name = name.to_string();
    let service_name = T::SERVICE_NAME.to_string();
    let full_name = format!(
      "{}{}{}",
      service_name.clone(),
      Self::NAMESPACE_SEPERATOR,
      name
    );
    if let Some(proxy) = T::get_existing_proxy(full_name.clone()).await {
      return proxy.wait().await.clone();
    }

    let maybe_future = if create_at_server {
      let maybe_future = MaybeFuture::new(Box::pin({
        let this = self.clone();
        async move {
          this.create_proxy(
            name.clone(),
            service_name.clone(),
            Box::pin({
              let this = this.clone();
              let name = name.clone();
              let service_name = service_name.clone();
              move |_| {
                Box::pin({
                  let this = this.clone();
                  let name = name.clone();
                  let service_name = service_name.clone();
                  async move {
                    Box::new(Arc::new(this.initialize_local_proxy(name.clone(), service_name.clone(), create_at_server).await))
                  }
                })
              }
            }),
          )
              .await
        }
      }));
      T::register_proxy(full_name, maybe_future.clone()).await;
      maybe_future
    } else {
      todo!()
    };
    maybe_future.wait().await
  }

  async fn initialize_local_proxy<T: Proxy + Sized + Send + Sync>(
    &self,
    name: String,
    service_name: String,
    create_at_server: bool,
  ) -> T {
    //todo: Add Map near cache proxy
    //todo: Add multimap proxy
    //todo: add reliabletopic proxy
    //todo: add flake id generator proxy

    T::create_proxy(ProxyBase::new(name, service_name, self.partition_service.clone(), self.invocation_service.clone(), self.serialization_service.clone())).await
  }

  async fn create_proxy<T: Proxy + Sized>(
    &self,
    name: String,
    service_name: String,
    handler: Pin<
      Box<
        dyn Send
        + Sync
        + Fn(ClientMessage) -> Pin<Box<dyn Send + Sync + Future<Output=Box<Arc<T>>>>>,
      >,
    >,
  ) -> T {
    let request = ClientCreateProxyCodec::encode_request(&name, &service_name).await;
    self
        .invocation_service
        .invoke_on_random_target(&*self.connection_registry.clone(), request, handler)
        .await
        .deref()
        .deref()
        .clone()
  }
}
