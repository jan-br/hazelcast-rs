use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::proxy::map_proxy::MapProxy;
use crate::serialization::serializable::Serializable;
use crate::util::observable_weak_arc::{ObservableArc, ObservableWeak};

struct WeakMapProxyInner<K: Serializable + Clone + Send + Sync + Eq + PartialEq + Hash + 'static, V: Serializable + Clone + Send + Sync + 'static> {
  inner: MapProxy<K, V>,
  own: HashMap<K, ObservableWeak<V>>,
}

#[derive(Clone)]
pub struct WeakMapProxy<K: Serializable + Clone + Send + Sync + Eq + PartialEq + Hash + 'static, V: Serializable + Clone + Send + Sync + 'static> {
  inner: Arc<Mutex<WeakMapProxyInner<K, V>>>,
}

impl<K: Serializable + Clone + Send + Sync + Eq + PartialEq + Hash + 'static, V: Serializable + Clone + Send + Sync + 'static> From<MapProxy<K, V>> for WeakMapProxy<K, V> {
  fn from(inner: MapProxy<K, V>) -> Self {
    Self {
      inner: Arc::new(Mutex::new(WeakMapProxyInner {
        inner,
        own: HashMap::new(),
      }))
    }
  }
}

impl<K: Serializable + Send + Sync + Clone + Eq + PartialEq + Hash + 'static, V: Serializable + 'static + Clone + Send + Sync> WeakMapProxy<K, V> {
  pub async fn put_weak_value(&self, key: impl Into<K>, value: ObservableArc<V>) {
    let key = key.into();
    let cloned_value = value.as_ref().clone().deref().deref().clone();
    {
      let mut inner = self.inner.lock().await;
      inner.inner.put(key.clone(), cloned_value.clone()).await;
      inner.own.insert(key.clone(), value.downgrade());
    }
    let inner = self.inner.clone();
    value.register_drop_callback(move |_| {
      let inner = inner.clone();
      let key = key.clone();
      tokio::spawn(async move {
        let mut inner = inner.lock().await;
        inner.own.remove(&key);
        inner.inner.remove(key).await;
      });
    }).await;
  }

  pub async fn get(&self, key: impl Into<K>) -> Option<V> {
    self.inner.lock().await.inner.get(key).await
  }

  pub async fn get_own(&self, key: impl Into<K>) -> Option<ObservableArc<V>> {
    let key = key.into();
    let mut inner = self.inner.lock().await;
    match inner.own.get(&key) {
      None => None,
      Some(value) => value.upgrade()
    }
  }
}
