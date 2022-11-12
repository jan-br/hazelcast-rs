use std::sync::Arc;
use tokio::sync::{RwLock, RwLockWriteGuard};
use crate::ClientNetworkConfig;
use crate::config::connection::ConnectionStrategyConfig;
use crate::config::retry::ClientRetryConfig;
use crate::config::security::SecurityConfig;
use crate::config::serialization::SerializationConfig;

pub mod network;
pub mod retry;
pub mod connection;
pub mod serialization;
pub mod security;

#[derive(Default)]
pub struct ClientConfig {
  pub network: Arc<RwLock<ClientNetworkConfig>>,
  pub retry: Arc<RwLock<ClientRetryConfig>>,
  pub connection_strategy: Arc<RwLock<ConnectionStrategyConfig>>,
  pub security: Arc<RwLock<SecurityConfig>>,
  pub cluster_name: String,
  pub client_name: String,
  pub serialization: Arc<RwLock<SerializationConfig>>,
}

impl ClientConfig {
  pub fn cluster_name(mut self, cluster_name: String) -> Self {
    self.cluster_name = cluster_name;
    self
  }

  pub async fn network<F: FnOnce(RwLockWriteGuard<ClientNetworkConfig>)>(self, callback: F) -> Self {
    let network = self.network.write().await;
    callback(network);
    self
  }

  pub async fn retry<F: FnOnce(RwLockWriteGuard<ClientRetryConfig>)>(self, callback: F) -> Self {
    let retry = self.retry.write().await;
    callback(retry);
    self
  }

  pub async fn security<F: FnOnce(RwLockWriteGuard<SecurityConfig>)>(self, callback: F) -> Self {
    let security = self.security.write().await;
    callback(security);
    self
  }
}