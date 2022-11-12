use std::collections::HashMap;
use std::sync::Arc;
use num_traits::abs;
use tokio::sync::RwLock;
use uuid::Uuid;
use crate::network::connection::Connection;
use crate::serialization::heap_data::HeapData;
use crate::serialization::service::SerializationServiceV1;

pub struct PartitionTable {
  pub connection: Option<Connection>,
  pub partition_state_version: i32,
  pub partitions: HashMap<u64, Uuid>,
}

impl PartitionTable {
  pub fn new() -> Self {
    Self {
      connection: None,
      partition_state_version: -1,
      partitions: HashMap::new(),
    }
  }
}

pub struct PartitionService {
  pub partition_table: PartitionTable,
  pub partition_count: RwLock<i32>,
  pub serialization_service: Arc<SerializationServiceV1>,
}

impl PartitionService {
  pub fn new(serialization_service: Arc<SerializationServiceV1>) -> PartitionService {
    Self {
      serialization_service,
      partition_count: RwLock::new(0),
      partition_table: PartitionTable::new(),
    }
  }

  pub async fn check_and_set_partition_count(&self, new_partition_count: i32) -> bool {
    let mut partition_count = self.partition_count.write().await;
    if *partition_count == 0 {
      *partition_count = new_partition_count;
      true
    } else {
      *partition_count == new_partition_count
    }
  }

  pub async fn get_partition_id(&self, key: HeapData) -> i32{
    let partition_count = self.partition_count.read().await;
    if *partition_count == 0 {
      todo!()
    }

    abs(key.get_partition_hash() as i32) % *partition_count
  }
}