use std::any::TypeId;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::core::distributed_object::DistributedObject;
use crate::proxy::base::{ProxyBase, ProxyBaseLogic};
use crate::serialization::schema::Schema;
use crate::util::maybe_future::MaybeFuture;

pub mod topic;
pub mod imap;
pub mod manager;
pub mod map_proxy;
pub mod base;
pub mod entry_event;
pub mod event_type;
pub mod multimap_proxy;
pub mod weak_map_proxy;
pub mod strong_map_proxy;
pub mod registry_proxy;
pub mod weak_registry_proxy;

pub trait Proxy: ProxyBaseLogic + 'static {
  const SERVICE_NAME: &'static str;
  fn get_proxies() -> Arc<RwLock<HashMap<String, Box<MaybeFuture<Self>>>>>;
  fn create_proxy(base: ProxyBase) -> Pin<Box<dyn Future<Output=Self> + Send + Sync>>;
}