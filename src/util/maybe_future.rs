use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct MaybeFuture<T: Clone + Send + Sync> {
  resolved: Arc<Mutex<Option<T>>>,
  future: Arc<Mutex<Option<Pin<Box<dyn Future<Output=T> + Send + Sync>>>>>,
}

impl<T: Clone + Send + Sync> MaybeFuture<T> {
  pub fn new(future: Pin<Box<dyn Future<Output=T> + Send + Sync>>) -> Self {
    MaybeFuture {
      resolved: Arc::new(Mutex::new(None)),
      future: Arc::new(Mutex::new(Some(future))),
    }
  }

  pub async fn wait(&self) -> T {
    let mut resolved = self.resolved.lock().await;
    if resolved.is_none() {
      *resolved = Some(self.future.lock().await.take().unwrap().await);
    }
    resolved.deref().as_ref().unwrap().clone()
  }
}
