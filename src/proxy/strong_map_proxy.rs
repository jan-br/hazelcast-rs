use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use crate::proxy::map_proxy::MapProxy;
use crate::serialization::serializable::Serializable;
use crate::util::observable_weak_arc::{ObservableArc, ObservableWeak};

#[derive(Clone)]
pub struct StrongMapProxy<K: Serializable + Eq + PartialEq + Hash + Send + Sync + Clone + 'static, V: Serializable + 'static + Clone + Send + Sync> {
  inner: ObservableArc<MapProxy<K, V>>,
  own: Arc<Mutex<HashSet<K>>>,
}

impl<K: Serializable + Eq + PartialEq + Hash + Send + Sync + Clone + 'static, V: Serializable + 'static + Clone + Send + Sync> From<MapProxy<K, V>> for StrongMapProxy<K, V> {
  fn from(inner: MapProxy<K, V>) -> Self {
    let inner = ObservableArc::from(inner);
    let own = Arc::new(Mutex::new(HashSet::new()));
    tokio::spawn({
      let inner = inner.clone();
      let own = own.clone();
      async move {
        inner.register_drop_callback({
          let own = own.clone();
          move |this| {
            let this = this.clone();
            tokio::spawn(async move {
              let own = std::mem::take(&mut *own.lock().await);
              for key in own {
                this.remove(key).await;
              }
            });
          }
        }).await
      }
    });
    Self {
      inner,
      own,
    }
  }
}


impl<K: Serializable + Eq + PartialEq + Hash + Send + Sync + Clone + 'static, V: Serializable + 'static + Clone + Send + Sync> StrongMapProxy<K, V> {
  pub async fn put_strong_value(&self, key: impl Into<K>, value: impl Into<V>) {
    let key = key.into();
    let value = value.into();
    self.inner.put(key.clone(), value.clone()).await;
    self.own.lock().await.insert(key.clone());
  }
}