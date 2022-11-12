use std::cmp::min;
use std::time::Duration;
use tokio::time::Instant;

pub struct WaitStrategy {
  initial_backoff: Duration,
  max_backoff: Duration,
  multiplier: f32,
  jitter: f32,
  cluster_connect_timeout: Option<Duration>,
  attempt: u32,
  current_backoff: Duration,
  cluster_connect_attempt_begin: Instant,
}

impl WaitStrategy {
  pub fn new(initial_backoff: Duration, max_backoff: Duration, multiplier: f32, jitter: f32, cluster_connect_timeout: Option<Duration>) -> Self {
    WaitStrategy {
      initial_backoff,
      max_backoff,
      multiplier,
      jitter,
      cluster_connect_timeout,
      attempt: 0,
      current_backoff: initial_backoff,
      cluster_connect_attempt_begin: Instant::now(),
    }
  }

  pub fn reset(&mut self) {
    self.attempt = 0;
    self.cluster_connect_attempt_begin = Instant::now();
    self.current_backoff = min(self.max_backoff, self.initial_backoff);
  }

  pub async fn sleep(&mut self) -> bool {
    self.attempt += 1;
    let current_time = Instant::now();
    let time_passed = current_time - self.cluster_connect_attempt_begin;
    if self.cluster_connect_timeout.is_some() && time_passed > self.cluster_connect_timeout.unwrap() {
      return false;
    }
    let actual_sleep_time = self.current_backoff.as_millis() + ((self.current_backoff.as_millis() as f32) * self.jitter * (2_f32 * rand::random::<f32>() - 1_f32)) as u128;
    let actual_sleep_time = min(actual_sleep_time, self.cluster_connect_timeout.unwrap_or(Duration::from_millis(0)).as_millis() - time_passed.as_millis());
    tokio::time::sleep(Duration::from_millis(actual_sleep_time as u64)).await;
    let next_current_backoff = (self.current_backoff.as_millis() as f32 * self.multiplier).round();
    self.current_backoff = min(Duration::from_millis(next_current_backoff as u64), Duration::from_millis(self.max_backoff.as_millis() as u64));
    true
  }
}