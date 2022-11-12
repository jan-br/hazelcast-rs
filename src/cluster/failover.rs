use std::future::Future;
use std::sync::Arc;
use futures::FutureExt;
use tokio::sync::RwLock;
use crate::{ClientConfig, DefaultAddressProvider};
use crate::cluster::candidate::CandidateClusterContext;

pub struct ClusterFailoverService {
  contexts: RwLock<Vec<Arc<CandidateClusterContext>>>,
  index: RwLock<usize>,
  max_try_count: u32,
}

impl ClusterFailoverService {
  pub fn new(configs: Vec<Arc<ClientConfig>>) -> Self {
    ClusterFailoverService {
      contexts: RwLock::new(configs.into_iter().map(|config| Arc::new(CandidateClusterContext::new(config.clone(), Arc::new(DefaultAddressProvider::new(config.network.clone()))))).collect()),
      index: RwLock::new(0),
      max_try_count: 0,
    }
  }

  pub async fn current(&self) -> Arc<CandidateClusterContext> {
    let contexts = self.contexts.read().await;
    let index = self.index.read().await;
    contexts.get(*index % contexts.len()).unwrap().clone()
  }

  pub async fn next(&self) -> Arc<CandidateClusterContext> {
    let contexts = self.contexts.read().await;
    let mut index = self.index.write().await;
    *index = (*index + 1) % contexts.len();
    contexts.get(*index).unwrap().clone()
  }

  pub async fn try_next_cluster<F: Fn(Arc<CandidateClusterContext>) -> R, R: Future<Output=bool>>(&self, function: F) -> bool {
    self.do_try_next_cluster(0, function).await
  }

  async fn do_try_next_cluster<F: Fn(Arc<CandidateClusterContext>) -> R, R: Future<Output=bool>>(&self, try_count: u32, function: F) -> bool {
    //todo: Add lifecycle service isRunning check

    let mut try_count = 0;

    loop {
      if try_count > self.max_try_count {
        return false;
      }
      let connected = function(self.next().await).await;
      if connected {
        return true;
      }
      try_count += 1
    }
  }
}