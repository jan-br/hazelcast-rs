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