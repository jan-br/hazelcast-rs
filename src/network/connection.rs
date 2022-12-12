use std::any::Any;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use chrono::NaiveDateTime;
use event_listener_primitives::Bag;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;
use crate::build_info::BuildInfo;
use crate::connection::address::Address;
use crate::invocation::{Invocation, InvocationReturnValue};
use crate::network::client_message_reader::ClientMessageReader;
use crate::network::fragmented_client_message_handler::FragmentedClientMessageHandler;
use crate::protocol::client_message::ClientMessage;

#[derive(Clone)]
pub struct Connection {
  pub remote_address: Arc<Mutex<Option<Arc<Address>>>>,
  pub remote_uuid: Arc<Mutex<Option<Uuid>>>,
  pub closed_time: Arc<Mutex<Option<NaiveDateTime>>>,
  pub closed_cause: Arc<Mutex<Option<String>>>,
  pub closed_reason: Arc<Mutex<Option<String>>>,
  pub write_half: Arc<Mutex<OwnedWriteHalf>>,
  pub read_half: Arc<Mutex<OwnedReadHalf>>,
  pub read_callback: Arc<RwLock<Option<Pin<Box<dyn Fn(ClientMessage) -> Pin<Box<dyn Future<Output=()> + Send + Sync>> + Send + Sync>>>>>,
  pub fragmented_message_handler: Arc<Mutex<FragmentedClientMessageHandler>>,
  pub connected_server_version: Arc<Mutex<Option<i32>>>,
  pub connection_id: i32,
  message_reader: Arc<Mutex<ClientMessageReader>>,
}

impl Connection {
  pub fn new(remote_address: Arc<Address>, write_half: OwnedWriteHalf, read_half: OwnedReadHalf, connection_id: i32) -> Self {
    Connection {
      closed_time: Arc::new(Mutex::new(None)),
      closed_cause: Arc::new(Mutex::new(None)),
      remote_uuid: Arc::new(Mutex::new(None)),
      remote_address: Arc::new(Mutex::new(Some(remote_address))),
      write_half: Arc::new(Mutex::new(write_half)),
      read_half: Arc::new(Mutex::new(read_half)),
      read_callback: Arc::new(RwLock::new(None)),
      message_reader: Arc::new(Mutex::new(ClientMessageReader::new())),
      fragmented_message_handler: Arc::new(Mutex::new(FragmentedClientMessageHandler::new())),
      connected_server_version: Arc::new(Mutex::new(None)),
      closed_reason: Arc::new(Mutex::new(None)),
      connection_id
    }
  }

  pub async fn start_reader(&self) {
    let mut read_half = self.read_half.lock().await;
    let mut buffer = [0; 1024];
    while let Ok(n) = read_half.read(&mut buffer).await {
      if n == 0 {
        println!("Connection closed");
        break;
      }
      let data = buffer[..n].to_vec();

      let mut message_reader = self.message_reader.lock().await;
      message_reader.append(data);
      let mut client_message = message_reader.read().await;
      while let Some(message) = &client_message {
        if message.start_frame.as_ref().unwrap().has_unfragmented_message_flag().await {
          let reader = self.read_callback.read().await;
          if let Some(callback) = &*reader {
            dbg!(message.clone().get_total_length().await);
            callback(message.clone()).await;
          }
        } else {
          let mut fragmented_message_handler = self.fragmented_message_handler.lock().await;
          fragmented_message_handler.handle_fragmented_message(client_message.take().unwrap()).await;
        }
        client_message = message_reader.read().await;
      }
      //todo: Add byte counter statistic
      // self.increment_bytes_read_fn(buffer.len());
    }
    todo!()
  }

  pub async fn set_read_callback(&self, callback: Pin<Box<dyn Fn(ClientMessage) -> Pin<Box<dyn Future<Output=()> + Send + Sync>> + Send + Sync>>) {
    *self.read_callback.write().await = Some(callback);
  }

  pub async fn write<R: InvocationReturnValue + Clone>(&self, invocation: Arc<RwLock<Invocation<R>>>) {
    let mut invocation = invocation.write().await;
    let mut write_half = self.write_half.lock().await;
    let buffer = invocation.request.to_buffer().await;
    write_half.write_all(&buffer).await.unwrap();
  }

  pub async fn set_connected_server_version(&self, version: String) {
    *self.connected_server_version.lock().await = Some(BuildInfo::calculate_server_version_from_string(Some(version)));
  }

  pub async fn set_remote_address(&self, address: Option<Address>) {
    *self.remote_address.lock().await = address.map(Arc::new);
  }

  pub async fn set_remote_uuid(&self, uuid: Option<Uuid>) {
    *self.remote_uuid.lock().await = uuid;
  }
  pub async fn close(&self, reason: String, cause: Option<String>) {
    let mut closed_time = self.closed_time.lock().await;
    if closed_time.is_some() {
      return;
    }
    *closed_time = Some(chrono::offset::Utc::now().naive_utc());

    *self.closed_reason.lock().await = Some(reason);
    *self.closed_cause.lock().await = cause;

    //todo: Log close
    todo!()
  }
}