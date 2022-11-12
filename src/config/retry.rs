use std::time::Duration;

#[derive(Default)]
pub struct ClientRetryConfig {
  pub initial_backoff: Duration,
  pub max_backoff: Duration,
  pub multiplier: f32,
  pub jitter: f32,
  pub cluster_connect_timeout: Option<Duration>,
}