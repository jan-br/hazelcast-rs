use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use chrono::{Duration, NaiveDateTime};
use uuid::Uuid;
use crate::invocation::service::InvocationService;
use crate::network::connection::Connection;
use crate::protocol::client_message::ClientMessage;
use crate::util::future::DeferredFuture;

pub mod service;
pub mod murmur;

pub trait InvocationReturnValue: Send + Sync {}

impl<T: Send + Sync> InvocationReturnValue for T {}

pub struct Invocation<R: InvocationReturnValue + Send + Sync> {
  pub invocation_service: Arc<InvocationService>,
  pub request: ClientMessage,
  pub partition_id: i32,
  pub uuid: Option<Uuid>,
  pub deadline: NaiveDateTime,
  pub connection: Option<Connection>,
  pub send_connection: Option<Connection>,
  pub pending_response_message: Option<ClientMessage>,
  pub backups_acks_received: u8,
  pub backups_acks_expected: u8,
  pub pending_response_received: NaiveDateTime,
  pub invoke_count: i32,
  pub urgent: bool,
  pub deferred: Option<DeferredFuture<R, String>>,
  pub handler: Option<Pin<Box<dyn Send + Sync + Fn(ClientMessage) -> Pin<Box<dyn Send + Sync + Future<Output=R>>>>>>,
}

impl<R: InvocationReturnValue + Send + Sync> Invocation<R> {
  pub fn new(invocation_service: Arc<InvocationService>, request: ClientMessage) -> Self {
    Self {
      deadline: chrono::Utc::now().naive_utc() + invocation_service.invocation_timeout,
      invocation_service,
      request,
      partition_id: -1,
      uuid: None,
      connection: None,
      send_connection: None,
      pending_response_message: None,
      invoke_count: 0,
      pending_response_received: NaiveDateTime::default(),
      backups_acks_expected: u8::MAX,
      backups_acks_received: 0,
      urgent: false,
      deferred: None,
      handler: None,
    }
  }

  pub fn new_with_custom_timeout(invocation_service: Arc<InvocationService>, request: ClientMessage, timeout: Duration) -> Self {
    Self {
      deadline: chrono::Utc::now().naive_utc() + timeout,
      invocation_service,
      request,
      partition_id: -1,
      uuid: None,
      connection: None,
      send_connection: None,
      pending_response_message: None,
      invoke_count: 0,
      pending_response_received: NaiveDateTime::default(),
      backups_acks_expected: u8::MAX,
      backups_acks_received: 0,
      urgent: false,
      deferred: None,
      handler: None,
    }
  }

  pub async fn notify(&mut self, client_message: ClientMessage) {
    let expected_backups = client_message.get_number_of_backup_acks().await;
    if expected_backups > self.backups_acks_received {
      self.pending_response_received = chrono::Utc::now().naive_utc();
      self.backups_acks_expected = expected_backups;
      self.pending_response_message = Some(client_message.clone());
    }
    self.complete(client_message).await;
  }

  pub async fn complete(&mut self, client_message: ClientMessage) {
    if let Some(handler) = self.handler.take() {
      let result = handler(client_message).await;
      if let Some(mut deferred) = self.deferred.take() {
        deferred.resolve(result).await;
      }
    }
    self.invocation_service.deregister_invocation(self.request.get_correlation_id().await).await;
  }
}