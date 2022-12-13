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
  pub bag: Arc<RwLock<Bag<Arc<dyn Fn(&LifecycleState) + Send + Sync>, LifecycleState>>>,
  pub active: Arc<RwLock<bool>>
}

impl LifecycleService {
  pub fn new() -> Self {
    LifecycleService {
      bag: Arc::new(RwLock::new(Bag::default())),
      active: Arc::new(RwLock::new(false))
    }
  }

  pub async fn start(&self) {
    self.emit_lifecycle_event(LifecycleState::Starting).await;
    *self.active.write().await = true;
    self.emit_lifecycle_event(LifecycleState::Started).await;
  }


  pub async fn emit_lifecycle_event(&self, state: LifecycleState) {
    //todo: log
    self.bag.read().await.call_simple( &state)
  }

  pub async fn is_running(&self) -> bool {
    self.active.read().await.clone()
  }
}