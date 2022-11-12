use std::sync::Arc;
use crate::{ClientConfig, DefaultAddressProvider};

pub struct CandidateClusterContext {
  pub config: Arc<ClientConfig>,
  pub address_provider: Arc<DefaultAddressProvider>,
}

impl CandidateClusterContext {
  pub fn new(config: Arc<ClientConfig>, address_provider: Arc<DefaultAddressProvider>) -> Self {
    CandidateClusterContext {
      config,
      address_provider,
    }
  }
}