use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::config::network::ClientNetworkConfig;
use crate::connection::address::{Address, Addresses};

pub struct DefaultAddressProvider {
  network_config: Arc<RwLock<ClientNetworkConfig>>,

}

impl DefaultAddressProvider {
  pub fn new(network_config: Arc<RwLock<ClientNetworkConfig>>) -> Self {
    DefaultAddressProvider {
      network_config,
    }
  }

  pub async fn load_addresses(&self) -> Addresses {
    let mut network_config = self.network_config.write().await;
    let cluster_members = &mut network_config.cluster_members;
    if cluster_members.is_empty() {
      cluster_members.push(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 5701)));
    }
    let primary = cluster_members.get(0).unwrap();
    let mut addresses = Addresses::new();

    addresses.primary.push(Arc::new(Address {
      host: primary.ip().to_string(),
      port: primary.port() as i32,
      scope: Some(0),
    }));

    for cluster_member in cluster_members[1..].iter() {
      addresses.secondary.push(Arc::new(Address {
        host: cluster_member.ip().to_string(),
        port: cluster_member.port() as i32,
        scope: Some(0),
      }));
    }
    addresses
  }

  pub fn translate(&self, address: Arc<Address>) -> Option<Arc<Address>> {
    Some(address)
  }
}