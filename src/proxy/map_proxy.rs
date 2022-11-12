use std::any::Any;
use std::collections::HashMap;
use std::future::Future;
use std::marker::PhantomData;
use std::mem::transmute;
use std::pin::Pin;
use std::sync::Arc;
use lazy_static::lazy_static;
use tokio::sync::RwLock;
use crate::proxy::base::{HasProxyBase, ProxyBase};
use crate::proxy::Proxy;
use crate::serialization::heap_data::HeapData;
use crate::serialization::serializable::Serializable;
use crate::util::maybe_future::MaybeFuture;

#[derive(Clone)]
pub struct MapProxy<K: Serializable, V: Serializable> {
  base: ProxyBase,
  phantom: PhantomData<(K, V)>,
}

impl<K: Serializable + Clone + 'static, V: Serializable + 'static> MapProxy<K, V> {
  pub fn new(base: ProxyBase) -> Self {
    MapProxy {
      base,
      phantom: PhantomData::default(),
    }
  }

  pub fn get(&self, key: &K) {
    let key_data = self.base.to_data(Box::new(key.clone()));
    // self.get_internal(key_data);
  }

  // fn get_internal(&self: &Arc<Self>, key_data: HeapData) {
  //   self.base.encode_invoke_on_key(key_data, |value| {
  //     todo!()
  //   }, |response| {
  //     todo!()
  //   });
  // }
}

trait AnySend: Any + Send + Sync {}

impl<K: Clone + Send + Sync + Serializable + 'static, V: Clone + Send + Sync + Serializable + 'static> Proxy for MapProxy<K, V> {
  const SERVICE_NAME: &'static str = "hz:impl:mapService";
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

impl<K: Send + Sync + Serializable + 'static, V: Send + Sync + Serializable + 'static> HasProxyBase for MapProxy<K, V> {
  fn get_proxy_base(&self) -> &ProxyBase {
    &self.base
  }
}