use std::any::Any;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::serialization::service::SerializationServiceV1;

pub struct ReadOnlyLazyList<T> {
  internal_list: Arc<Mutex<Vec<Box<dyn Any>>>>,
  serializatiion_service: Arc<SerializationServiceV1>,
  _phantom: PhantomData<T>,
}

impl<T> ReadOnlyLazyList<T> {
  pub async fn get(&self, index: usize) -> Option<T> {
    let mut internal_list = self.internal_list.lock().await;
    internal_list.get_mut(index).map(|item| {
      todo!()
    })
  }
}