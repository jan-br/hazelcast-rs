use std::sync::Arc;
use async_trait_with_sync::async_trait;
use tokio::sync::RwLock;
use crate::invocation::InvocationReturnValue;

#[async_trait]
pub trait DistributedObject: InvocationReturnValue{
  fn get_partition_key(&self) -> String;
  fn get_name(&self) -> String;
  fn get_service_name(&self) -> String;
  async fn destroy(self);
}
