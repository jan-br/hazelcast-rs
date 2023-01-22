use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;
use hazelcast_rs::client::HazelcastClient;
use hazelcast_rs::config::ClientConfig;

#[tokio::main]
async fn main() {
  let client_config = Arc::new(ClientConfig::default()
    .cluster_name("hello-world".to_string())
    .network(|mut network| {
      network.cluster_members.push(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 5701)));
    })
    .await);
  let mut client = HazelcastClient::new(client_config).await;
  client.start().await;

  let map = client.get_map::<String, String>("something").await;
  map.put("test", "blub").await;
  let option = map.get("test").await;
  println!("{:?}", option);
  map.remove("test").await;
}