use crate::config::retry::ClientRetryConfig;

#[derive(Default)]
pub struct ConnectionStrategyConfig {
  pub async_start: bool,
  pub reconnect_mode: ReconnectMode,
  connection_retry: ClientRetryConfig,
}

impl ConnectionStrategyConfig {
  pub fn new(
    async_start: bool,
    reconnect_mode: ReconnectMode,
    connection_retry: ClientRetryConfig,
  ) -> Self {
    ConnectionStrategyConfig {
      async_start,
      reconnect_mode,
      connection_retry,
    }
  }
}

#[derive(Clone, Default, Eq, PartialEq)]
pub enum ReconnectMode {
  Off,
  On,
  #[default]
  Async,
}