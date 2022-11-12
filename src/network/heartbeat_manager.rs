use chrono::Duration;

pub struct HeartbeatManager {
  pub hartbeat_timeout: Duration,
}

impl HeartbeatManager {
  pub fn new() -> Self {
    Self {
      hartbeat_timeout: Duration::seconds(10)
    }
  }
}