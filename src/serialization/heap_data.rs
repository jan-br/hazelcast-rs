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
    todo!()
  }

  pub fn new(payload: Vec<u8>) -> Self {
    if payload.len() > 0 && payload.len() < Self::HEAP_DATA_OVERHEAD as usize {
      todo!()
    }
    Self {
      payload,
    }
  }

  pub fn get_partition_hash(&self) -> i32 {
    // if self.has_partition_hash() {
    //   byteorder::BE::read_i32(self.payload[Self::PARTITION_HASH_OFFSET as usize..].as_slice())
    // }else{
    //   self.hash_code()
    // }
    todo!()
  }
}