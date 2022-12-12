#![feature(async_closure)]
#![feature(fn_traits)]
#![feature(trait_upcasting)]
#![feature(trait_alias)]

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;
use std::time::Duration;
use crate::client::HazelcastClient;
use crate::config::ClientConfig;
use crate::config::network::ClientNetworkConfig;
use crate::connection::address::provider::DefaultAddressProvider;
use crate::proxy::event_type::EventType;

pub mod cluster;
pub mod config;
pub mod connection;
pub mod client;
pub mod network;
pub mod protocol;
pub mod util;
pub mod codec_builtin;
pub mod core;
pub mod serialization;
pub mod invocation;
pub mod partition_service;
pub mod proxy;
pub mod build_info;
pub mod lifecycle_service;
pub mod listener;

pub mod codec {
  pub mod client_authentication_codec;
  pub mod client_add_cluster_view_listener_codec;
  pub mod client_fetch_schema_codec;
  pub mod client_create_proxy_codec;
  pub mod map_get_codec;
  pub mod map_put_codec;
  pub mod map_add_entry_listener_codec;
  pub mod map_remove_entry_listener_codec;

  pub mod custom {
    pub mod address_codec;
    pub mod distributed_object_info_codec;
    pub mod member_version_codec;
    pub mod endpoint_qualifier_codec;
    pub mod schema_codec;
    pub mod member_info_codec;
    pub mod field_descriptor_codec;
  }
}

#[tokio::main]
pub async fn main() {
  let client_config = Arc::new(ClientConfig::default()
    .cluster_name("hello-world".to_string())
    .network(|mut network| {
      network.cluster_members.push(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 5701)));
    })
    .await);
  let mut client = HazelcastClient::new(client_config).await;
  client.start().await;

  let test_map = client.get_map::<Option<String>, Option<String>>("test").await;
  let option = test_map.get(&Some("key".to_string())).await;
  test_map.add_entry_listener::<{EventType::ALL}>(|event| Box::pin(async move {
    println!("Event {}", event.key.unwrap().unwrap() );
  })).await;
  test_map.put(Some("key".to_string()), Some("value".to_string())).await;
  dbg!(option);
  tokio::time::sleep(Duration::from_secs(1000)).await;
}
