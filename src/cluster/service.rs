use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::ClientConfig;
use crate::cluster::failover::ClusterFailoverService;
use crate::core::member::{Member, MemberListSnapshot, MemberSelector};

pub struct ClusterService {
  member_list_snapshot: RwLock<MemberListSnapshot>,
  cluster_failover_service: Arc<ClusterFailoverService>,
}

impl ClusterService {
  pub fn new(config: Arc<ClientConfig>, cluster_failover_service: Arc<ClusterFailoverService>) -> ClusterService {
    ClusterService {
      member_list_snapshot: RwLock::new(MemberListSnapshot {
        version: -1,
        members: HashMap::new(),
        member_list: Vec::new(),
      }),
      cluster_failover_service,
    }
  }

  pub async fn get_members(&self, selector: Option<MemberSelector>) -> Vec<Arc<Member>> {
    let members = self.get_member_list().await;
    if selector.is_none() {
      members
    } else {
      let selector = selector.unwrap();
      members.into_iter().filter(|member| selector(member)).collect::<Vec<_>>()
    }
  }

  async fn get_member_list(&self) -> Vec<Arc<Member>> {
    let member_list_snapshot = self.member_list_snapshot.read().await;
    member_list_snapshot.member_list.clone()
  }
}