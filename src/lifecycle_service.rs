use std::sync::Arc;
use event_listener_primitives::Bag;
use tokio::sync::RwLock;

pub enum LifecycleState {
  Starting,
  Started,
  ShuttingDown,
  Connected,
  Disconnected,
  ChangedCluster
}

pub struct LifecycleService {
  pub bag: Arc<RwLock<Bag<Arc<dyn Fn(&LifecycleState) + Send + Sync>, LifecycleState>>>
}

impl LifecycleService {
  pub fn new() -> Self {
    LifecycleService {
      bag: Arc::new(RwLock::new(Bag::default()))
    }
  }
  pub async fn emit_lifecycle_event(&self, state: LifecycleState) {
    //todo: log
    self.bag.read().await.call_simple( &state)
  }
}