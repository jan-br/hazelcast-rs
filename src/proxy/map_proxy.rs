use std::any::Any;
use std::collections::HashMap;
use std::future::Future;
use std::marker::PhantomData;
use std::mem::transmute;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use lazy_static::lazy_static;
use tokio::sync::RwLock;
use uuid::Uuid;
use crate::codec::map_add_entry_listener_codec::MapAddEntryListenerCodec;

use crate::codec::map_get_codec::MapGetCodec;
use crate::codec::map_put_codec::MapPutCodec;
use crate::codec::map_remove_entry_listener_codec::MapRemoveEntryListenerCodec;
use crate::listener::message_codec::ListenerMessageCodec;
use crate::protocol::client_message::ClientMessage;
use crate::proxy::base::{HasProxyBase, ProxyBase};
use crate::proxy::entry_event::EntryEvent;
use crate::proxy::Proxy;
use crate::serialization::heap_data::HeapData;
use crate::serialization::serializable::Serializable;
use crate::util::maybe_future::MaybeFuture;

#[derive(Clone)]
pub struct MapProxy<K: Serializable, V: Serializable> {
  base: ProxyBase,
  phantom: PhantomData<(K, V)>,
}

pub trait MapListener<K, V, const FLAGS: i32> = Fn(EntryEvent<K, V>) -> Pin<Box<dyn Send + Sync + Future<Output=()>>> + Send + Sync + 'static;

struct EntryListenerCodec {
  name: String,
  include_value: bool,
  flags: i32,
}

impl EntryListenerCodec {
  pub fn new(name: String, include_value: bool, flags: i32) -> Self {
    Self {
      name,
      include_value,
      flags,
    }
  }
}

impl ListenerMessageCodec for EntryListenerCodec {
  fn encode_add_request<'a>(&'a self, local_only: &'a bool) -> Pin<Box<dyn Future<Output=ClientMessage> + Send + Sync + 'a>> {
    MapAddEntryListenerCodec::encode_request(&self.name, &self.include_value, &self.flags, local_only)
  }

  fn decode_add_response<'a>(&'a self, client_message: &'a mut ClientMessage) -> Pin<Box<dyn Future<Output=Uuid> + Send + Sync + 'a>> {
    MapAddEntryListenerCodec::decode_response(client_message)
  }

  fn encode_remove_request<'a>(&'a self, registration_id: &'a Uuid) -> Pin<Box<dyn Future<Output=ClientMessage> + Send + Sync + 'a>> {
    MapRemoveEntryListenerCodec::encode_request(&self.name, registration_id)
  }
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

  pub async fn add_entry_listener<const FLAGS: i32>(&self, listener: impl MapListener<K, V, FLAGS>) {
    let listener = Arc::new(listener);
    let cluster_service = self.base.cluster_service.clone();
    let base = self.base.clone();
    self.base.listener_service.register_listener(EntryListenerCodec::new(self.base.name.clone(), true, FLAGS), {
      move |mut client_message| {
        let listener = listener.clone();
        let cluster_service = cluster_service.clone();
        let base = base.clone();
        Box::pin(async move {
          MapAddEntryListenerCodec::handle(&mut client_message, Some(Box::pin({
            move |key, value, old_value, merging_value, event_type, uuid, number_of_affeced_entries| {
              let listener = listener.clone();
              let base = base.clone();
              let cluster_service = cluster_service.clone();
              Box::pin(async move {
                listener.call((EntryEvent::new(
                  base.name.clone(),
                  if let Some(key) = key { Some(*base.serialization_service.to_object(key).await) } else { None },
                  if let Some(value) = value { Some(*base.serialization_service.to_object(value).await) } else { None },
                  if let Some(old_value) = old_value { Some(*base.serialization_service.to_object(old_value).await) } else { None },
                  if let Some(merging_value) = merging_value { Some(*base.serialization_service.to_object(merging_value).await) } else { None },
                  cluster_service.get_member(uuid).await,
                ), )).await;
              })
            }
          }))).await;
        })
      }
    }).await;

    // self.base.listener_service.register_listener(EntryListenerCodec::new(self.base.name.clone(), true, FLAGS), {
    //   let name = self.base.name.clone();
    //   let listener = listener.clone();
    //   let cluster_service = self.base.cluster_service.clone();
    //   move |mut client_message| {
    //     let listener = listener.clone();
    //     let cluster_service = self.base.cluster_service.clone();
    //     // Box::pin(listener.call((client_message, )))
    //     Box::pin(async move {
    //       // MapAddEntryListenerCodec::handle(&mut client_message, Some(Box::pin(|_, _, _, _, _, _, _| Box::pin(async move { todo!() })))).await;
    //       todo!()
    //     })
    //   }
    // }).await;
  }


  pub async fn put(&self, key: K, value: V) {
    let key_data = self.base.to_data(Box::new(key));
    let value_data = self.base.to_data(Box::new(value));
    self.put_internal(key_data, value_data, None).await;
  }

  pub async fn put_with_ttl(&self, key: K, value: V, ttl: Duration) {
    let key_data = self.base.to_data(Box::new(key));
    let value_data = self.base.to_data(Box::new(value));
    self.put_internal(key_data, value_data, Some(ttl)).await;
  }


  async fn put_internal(&self, key_data: HeapData, value_data: HeapData, ttl: Option<Duration>) {
    self.base.encode_invoke_on_key(
      key_data.clone(),
      Box::pin({
        move |name| Box::pin({
          let key_data = key_data.clone();
          let value_data = value_data.clone();
          async move {
            MapPutCodec::encode_request(&name, &key_data, &value_data, &0, &ttl.map(|ttl|ttl.as_secs() as i64).unwrap_or(-1)).await
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