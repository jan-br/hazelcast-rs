use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::{Arc, Weak};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct ObservableArc<T: Clone + Send + 'static> {
  inner: Arc<DropAware<T>>,
}

impl<T: Clone + Send + Debug + 'static> Debug for ObservableArc<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    Debug::fmt(&self.inner, f)
  }
}

impl<T: Clone + Send + 'static> From<T> for ObservableArc<T> {
  fn from(value: T) -> Self {
    Self {
      inner: Arc::new(DropAware::new(value))
    }
  }
}

impl<T: Clone + Send + 'static> ObservableArc<T> {
  pub fn downgrade(&self) -> ObservableWeak<T> {
    ObservableWeak::new(Arc::downgrade(&self.inner))
  }
}

impl<T: Clone + Send + 'static> Deref for ObservableArc<T> {
  type Target = Arc<DropAware<T>>;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

#[derive(Clone)]
pub struct ObservableWeak<T: Clone + Send + 'static> {
  inner: Weak<DropAware<T>>,
}

impl<T: Clone + Send + 'static> ObservableWeak<T> {
  pub fn new(inner: Weak<DropAware<T>>) -> Self {
    Self {
      inner
    }
  }

  pub fn upgrade(&self) -> Option<ObservableArc<T>> {
    self.inner.upgrade().map(|inner| ObservableArc { inner })
  }
}

impl<T: Clone + Send + 'static> Deref for ObservableWeak<T> {
  type Target = Weak<DropAware<T>>;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

pub struct DropAware<T: Clone + Send + 'static> {
  inner: T,
  callbacks: Arc<Mutex<Vec<Box<dyn FnOnce(&T) + Send + Sync>>>>,
}

impl<T: Clone + Send + Debug + 'static> Debug for DropAware<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    Debug::fmt(&self.inner, f)
  }
}

impl<T: Clone + Send + 'static> DropAware<T> {
  pub fn new(inner: T) -> Self {
    Self { inner, callbacks: Arc::new(Mutex::new(vec![])) }
  }

  pub async fn register_drop_callback(&self, callback: impl FnOnce(&T) + Send + Sync + 'static) {
    self.callbacks.lock().await.push(Box::new(callback));
  }
}

impl<T: Clone + Send + 'static> Deref for DropAware<T> {
  type Target = T;
  fn deref(&self) -> &T {
    &self.inner
  }
}

impl<T: Clone + Send + 'static> Drop for DropAware<T> {
  fn drop(&mut self) {
    let inner_clone = self.inner.clone();
    let callbacks = self.callbacks.clone();
    tokio::spawn(async move {
      let inner_clone = inner_clone.clone();
      let callbacks = callbacks;
      let mut callbacks = callbacks.lock().await;

      for callback in std::mem::take(&mut *callbacks) {
        callback.call_once((&inner_clone, ));
      }
    });
  }
}