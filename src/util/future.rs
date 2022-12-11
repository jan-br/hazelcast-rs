use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::Mutex;

use tokio::sync::oneshot::{Receiver, Sender};

#[derive(Clone)]
pub struct DeferredFuture<T, E: Debug + Clone> {
  sender: Arc<Mutex<Option<Sender<Result<T, E>>>>>,
  receiver: Arc<Mutex<Option<Receiver<Result<T, E>>>>>,
  drop_blub: i32
}



impl<T, E> DeferredFuture<T, E> where E: Debug + Clone {
  pub async fn resolve(&mut self, value: T) {
    //todo: Error handling
    let mut sender = self.sender.lock().await;
    if let Some(sender) = sender.take() {
      let result = sender.send(Ok(value));
      result.map_err(|e| {

      }).unwrap();
    } else {
      todo!()
    }
  }

  pub async fn reject(&mut self, error: E) {
    //todo: Error handling
    let mut sender = self.sender.lock().await;
    if let Some(sender) = sender.take() {
      sender.send(Err(error));
    } else {
      todo!()
    }
  }

  pub async fn wait(&mut self) -> Result<T, E> {
    let mut receiver = self.receiver.lock().await;
    if let Some(receiver) = receiver.take() {
      receiver.await.unwrap()
    } else {
      todo!()
    }
  }
}


impl<T, E> Default for DeferredFuture<T, E> where E: Debug + Clone {
  fn default() -> Self {
    let (sender, receiver) = tokio::sync::oneshot::channel();
    Self {
      sender: Arc::new(Mutex::new(Some(sender))),
      receiver: Arc::new(Mutex::new(Some(receiver))),
      drop_blub: 1337
    }
  }
}