use std::collections::HashMap;
use std::future::Future;
use std::marker::PhantomData;
use std::mem::transmute;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use lazy_static::lazy_static;
use tokio::sync::RwLock;
use crate::codec::multi_map_get_codec::MultiMapGetCodec;
use crate::codec::multi_map_put_codec::MultiMapPutCodec;
use crate::codec::multi_map_remove_entry_codec::MultiMapRemoveEntryCodec;
use crate::proxy::base::{HasProxyBase, ProxyBase};
use crate::proxy::map_proxy::AnySend;
use crate::proxy::Proxy;
use crate::serialization::heap_data::HeapData;
use crate::serialization::serializable::Serializable;
use crate::util::maybe_future::MaybeFuture;

#[derive(Clone)]
pub struct MultiMapProxy<K: Serializable, V: Serializable> {
  base: ProxyBase,
  phantom: PhantomData<(K, V)>,
}

impl<K: Serializable + Send + Sync + Clone + 'static, V: Serializable + 'static + Clone + Send + Sync> MultiMapProxy<K, V> {
  pub fn new(
    base: ProxyBase,
  ) -> Self {
    MultiMapProxy {
      base,
      phantom: PhantomData::default(),
    }
  }

  pub async fn get(&self, key: impl Into<K>) -> Vec<V> {
    let key = key.into();
    let key_data = self.base.to_data(Box::new(key));
    self.get_internal(key_data).await
  }

  async fn get_internal(&self, key_data: HeapData) -> Vec<V> {
    self.base.encode_invoke_on_key(
      key_data.clone(),
      Box::pin(move |value| {
        let key_data = key_data.clone();

        Box::pin(async move {
          MultiMapGetCodec::encode_request(&value, &key_data, &0).await
        })
      }),
      Box::pin({
        let serialization_service = self.get_proxy_base().serialization_service.clone();

        move |mut response| {
          let serialization_service = serialization_service.clone();

          Box::pin(async move {
            let response = MultiMapGetCodec::decode_response(&mut response).await;
            let mut result: Vec<V> = vec![];
            for entry in response {
              result.push(*serialization_service.to_object(entry).await);
            }
            Box::new(Box::new(result))
          })
        }
      })).await
  }

  pub async fn remove(&self, key: impl Into<K>, value: impl Into<V>) -> bool {
    let key = key.into();
    let value = value.into();
    let key_data = self.base.to_data(Box::new(key));
    let value_data = self.base.to_data(Box::new(value));
    self.remove_internal(key_data, value_data).await
  }

  async fn remove_internal(&self, key_data: HeapData, value_data: HeapData) -> bool {
    self.base.encode_invoke_on_key(
      key_data.clone(),
      Box::pin({
        move |name| Box::pin({
          let key_data = key_data.clone();
          let value_data = value_data.clone();
          async move {
            MultiMapRemoveEntryCodec::encode_request(&name, &key_data, &value_data, &0).await
          }
        })
      }),
      Box::pin(|mut response| Box::pin(async move {
        Box::new(Box::new(MultiMapRemoveEntryCodec::decode_response(&mut response).await))
      })),
    ).await
  }

  pub async fn put(&self, key: impl Into<K>, value: impl Into<V>) {
    let key = key.into();
    let value = value.into();
    let key_data = self.base.to_data(Box::new(key));
    let value_data = self.base.to_data(Box::new(value));
    self.put_internal(key_data, value_data).await;
  }

  async fn put_internal(&self, key_data: HeapData, value_data: HeapData) {
    self.base.encode_invoke_on_key(
      key_data.clone(),
      Box::pin({
        move |name| Box::pin({
          let key_data = key_data.clone();
          let value_data = value_data.clone();
          async move {
            MultiMapPutCodec::encode_request(&name, &key_data, &value_data, &0).await
          }
        })
      }),
      Box::pin(|mut response| Box::pin(async move { Box::new(Box::new(MultiMapPutCodec::decode_response(&mut response).await)) })),
    ).await;
  }
}

impl<K: Clone + Send + Sync + Serializable + 'static, V: Clone + Send + Sync + Serializable + 'static> Proxy for MultiMapProxy<K, V> {
  const SERVICE_NAME: &'static str = "hz:impl:multiMapService";
  fn get_proxies() -> Arc<RwLock<HashMap<String, Box<MaybeFuture<Self>>>>> {
    lazy_static! {
      static ref PROXIES: Arc<RwLock<HashMap<String, Box<dyn AnySend>>>> = Arc::new(RwLock::new(HashMap::new()));
    }
    unsafe { transmute(PROXIES.clone()) }
  }
  fn create_proxy(base: ProxyBase) -> Pin<Box<dyn Future<Output=Self> + Send + Sync>> {
    Box::pin(async move {
      Self::new(base)
    })
  }
}

impl<K: Send + Sync + Serializable + 'static, V: Send + Sync + Serializable + 'static> HasProxyBase for MultiMapProxy<K, V> {
  fn get_proxy_base(&self) -> &ProxyBase {
    &self.base
  }
}