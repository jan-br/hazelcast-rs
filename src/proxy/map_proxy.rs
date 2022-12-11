use std::any::Any;
use std::collections::HashMap;
use std::future::Future;
use std::marker::PhantomData;
use std::mem::transmute;
use std::pin::Pin;
use std::sync::Arc;

use lazy_static::lazy_static;
use tokio::sync::RwLock;

use crate::codec::map_get_codec::MapGetCodec;
use crate::codec::map_put_codec::MapPutCodec;
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


impl<K: Serializable + Send + Sync + Clone + 'static, V: Serializable + 'static + Clone + Send + Sync> MapProxy<K, V> {
  pub fn new(
    base: ProxyBase,
  ) -> Self {
    MapProxy {
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
            MapPutCodec::encode_request(&name, &key_data, &value_data, &0, &0).await
          }
        })
      }),
      Box::pin(|mut response| Box::pin(async move { Box::new(Box::new(MapGetCodec::decode_response(&mut response).await)) })),
    ).await;
  }

  pub async fn get(&self, key: &K) -> Option<V> {
    let key_data = self.base.to_data(Box::new(key.clone()));
    self.get_internal(key_data).await.map(|data| *data)
  }
  async fn get_internal(&self, key_data: HeapData) -> Option<Box<V>> {
    self.base.encode_invoke_on_key(key_data.clone(), {
      Box::pin(move |value| {
        let key_data = key_data.clone();

        Box::pin(async move {
          MapGetCodec::encode_request(&value, &key_data, &0).await
        })
      })
    }, Box::pin({
      let serialization_service = self.get_proxy_base().serialization_service.clone();
      move |mut response| {
        let serialization_service = serialization_service.clone();
        Box::pin(async move {
          let serialization_service = serialization_service.clone();
          if let Some(data) = MapGetCodec::decode_response(&mut response).await {
            Box::new(Box::new(Some(serialization_service.to_object::<V>(data).await)))
          } else {
            Box::new(Box::new(None))
          }
        })
      }
    })).await
  }
}

pub trait AnySend: Any + Send + Sync {}

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