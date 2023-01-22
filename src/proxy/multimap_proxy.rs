use std::collections::HashMap;
use std::future::Future;
use std::marker::PhantomData;
use std::mem::transmute;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use lazy_static::lazy_static;
use tokio::sync::RwLock;
use crate::codec::multi_map_put_codec::MultiMapPutCodec;
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

  pub async fn put(&self, key: K, value: V) {
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