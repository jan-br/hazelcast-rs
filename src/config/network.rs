use std::net::SocketAddr;

#[derive(Clone, Default)]
pub struct ClientNetworkConfig {
    pub cluster_members: Vec<SocketAddr>,
    pub smart_routing: bool
}
