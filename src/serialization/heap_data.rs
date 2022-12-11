use std::cmp::max;
use std::io::Cursor;
use byteorder::ByteOrder;

#[derive(Clone)]
pub struct HeapData {
  payload: Vec<u8>,
}

impl HeapData {
  pub const PARTITION_HASH_OFFSET: i32 = 0;
  pub const TYPE_OFFSET: i32 = 4;
  pub const DATA_OFFSET: i32 = 8;
  pub const HEAP_DATA_OVERHEAD: i32 = Self::DATA_OFFSET;

  pub fn to_buffer(&self) -> Vec<u8> {
    self.payload.clone()
  }

  pub fn new(payload: Vec<u8>) -> Self {
    if payload.len() > 0 && payload.len() < Self::HEAP_DATA_OVERHEAD as usize {
      todo!()
    }
    Self {
      payload,
    }
  }

  pub fn total_size(&self) -> i32 {
    self.payload.len() as i32
  }

  pub fn get_type(&self) -> i32 {
    if self.total_size() == 0 {
      0
    } else {
      byteorder::BigEndian::read_i32(&self.payload[Self::TYPE_OFFSET as usize..Self::TYPE_OFFSET as usize + 4])
    }
  }

  pub fn has_partition_hash(&self) -> bool {
    self.payload.len() >= Self::HEAP_DATA_OVERHEAD as usize
      && byteorder::BE::read_i32(&self.payload[Self::PARTITION_HASH_OFFSET as usize..Self::PARTITION_HASH_OFFSET as usize + 4]) != 0
  }

  pub fn hash_code(&self) -> i32 {
    murmur3::murmur3_32(&mut Cursor::new(self.payload.clone()[Self::DATA_OFFSET as usize..Self::DATA_OFFSET as usize + self.data_size() as usize].to_vec()), 0x01000193).unwrap() as i32
  }

  pub fn data_size(&self) -> i32 {
    max(self.total_size() - Self::HEAP_DATA_OVERHEAD, 0)
  }

  pub fn get_partition_hash(&self) -> i32 {
    if self.has_partition_hash() {
      byteorder::BE::read_i32(&self.payload[Self::PARTITION_HASH_OFFSET as usize..Self::PARTITION_HASH_OFFSET as usize + 4])
    } else {
      self.hash_code()
    }
  }
}