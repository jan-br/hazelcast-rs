use std::any::Any;
use std::sync::Arc;
use crate::serialization::data::{DataInput, DataInputReadObject};
use crate::serialization::heap_data::HeapData;
use crate::serialization::serializable::{IdentifiedDataSerializable, IdentifiedDataSerializableSerialization};
use crate::serialization::service::SerializationServiceV1;
use crate::util::bits_util::BitsUtil;

pub struct ObjectDataInput {
  pub buffer: Vec<u8>,
  pub offset: usize,
  pub service: Arc<SerializationServiceV1>,
  pub pos: usize,
  pub big_endian: bool,
}

impl ObjectDataInput {
  pub fn new(buffer: Vec<u8>, offset: usize, serialization_service: Arc<SerializationServiceV1>, is_big_endian: bool) -> Self {
    ObjectDataInput {
      buffer,
      offset,
      service: serialization_service,
      pos: offset,
      big_endian: is_big_endian,
    }
  }
}

impl DataInput for ObjectDataInput {
  fn position(&self) -> usize {
    self.pos
  }

  fn set_position(&mut self, new_position: usize) -> usize {
    let old_pos = self.pos;
    self.pos = new_position;
    old_pos
  }

  fn read(&mut self) -> u8 {
    self.assert_available(BitsUtil::BYTE_SIZE_IN_BYTES, self.pos);
    self.pos += 1;
    BitsUtil::read_uint8(&self.buffer, self.pos)
  }

  fn read_pos(&mut self, pos: usize) -> u8 {
    self.assert_available(BitsUtil::BYTE_SIZE_IN_BYTES, pos);
    BitsUtil::read_uint8(&self.buffer, pos)
  }

  fn read_byte(&mut self) -> u8 {
    self.read()
  }

  fn read_byte_pos(&mut self, pos: usize) -> u8 {
    self.read_pos(pos)
  }

  fn read_boolean(&mut self) -> bool {
    self.read() == 1
  }

  fn read_boolean_pos(&mut self, pos: usize) -> bool {
    self.read_pos(pos) == 1
  }

  fn read_boolean_array(&mut self) -> Option<Vec<bool>> {
    self.read_array(Box::new(|this| this.read_boolean()))
  }

  fn read_boolean_array_pos(&mut self, pos: usize) -> Option<Vec<bool>> {
    self.read_array_pos(Box::new(|this| this.read_boolean()), pos)
  }

  fn read_byte_array(&mut self) -> Option<Vec<u8>> {
    let len = self.read_int();
    if len == BitsUtil::NULL_ARRAY_LENGTH {
      return None;
    }
    let buf = self.buffer[self.pos..self.pos + len as usize].to_vec();
    self.pos += len as usize;
    Some(buf)
  }

  fn read_byte_array_pos(&mut self, pos: usize) -> Option<Vec<u8>> {
    let backup_pos = self.pos;
    self.pos = pos;
    let len = self.read_int();
    if len == BitsUtil::NULL_ARRAY_LENGTH {
      self.pos = backup_pos;
      return None;
    }
    let buf = self.buffer[self.pos..self.pos + len as usize].to_vec();
    self.pos = backup_pos;
    Some(buf)
  }

  fn read_array<T>(&mut self, func: Box<dyn Fn(&mut Self) -> T>) -> Option<Vec<T>> {
    let len = self.read_int();
    if len == BitsUtil::NULL_ARRAY_LENGTH {
      return None;
    }
    let mut arr = vec![];
    for _ in 0..len {
      arr.push(func(self));
    }
    Some(arr)
  }

  fn read_array_pos<T>(
    &mut self,
    func: Box<dyn Fn(&mut Self) -> T>,
    pos: usize,
  ) -> Option<Vec<T>> {
    let backup_pos = self.pos;
    self.pos = pos;
    let len = self.read_int();
    if len == BitsUtil::NULL_ARRAY_LENGTH {
      self.pos = backup_pos;
      return None;
    }
    let mut arr = vec![];
    for _ in 0..len {
      arr.push(func(self));
    }
    self.pos = backup_pos;
    Some(arr)
  }

  fn read_int_array(&mut self) -> Option<Vec<i32>> {
    self.read_array(Box::new(|this| this.read_int()))
  }

  fn read_int(&mut self) -> i32 {
    self.assert_available(BitsUtil::INT_SIZE_IN_BYTES, self.pos);
    let ret = BitsUtil::read_int32(&mut self.buffer, self.pos, self.big_endian);
    self.pos += BitsUtil::INT_SIZE_IN_BYTES as usize;
    ret
  }

  fn read_long(&mut self) -> i64 {
    self.assert_available(BitsUtil::LONG_SIZE_IN_BYTES, self.pos);
    let ret = BitsUtil::read_int64(&mut self.buffer, self.pos, self.big_endian);
    self.pos += BitsUtil::LONG_SIZE_IN_BYTES as usize;
    ret
  }

  fn read_int_pos(&mut self, pos: usize) -> i32 {
    self.assert_available(BitsUtil::INT_SIZE_IN_BYTES, pos);
    BitsUtil::read_int32(&mut self.buffer, pos, self.big_endian)
  }

  fn read_float(&mut self) -> f32 {
    self.assert_available(BitsUtil::FLOAT_SIZE_IN_BYTES, self.pos);
    let ret = BitsUtil::read_float32(&mut self.buffer, self.pos, self.big_endian);
    self.pos += BitsUtil::FLOAT_SIZE_IN_BYTES as usize;
    ret
  }

  fn read_short(&mut self) -> i16 {
    self.assert_available(BitsUtil::SHORT_SIZE_IN_BYTES, self.pos);
    let ret = BitsUtil::read_int16(&mut self.buffer, self.pos, self.big_endian);
    self.pos += BitsUtil::INT_SIZE_IN_BYTES as usize;
    ret
  }

  fn read_double(&mut self) -> f64 {
    self.assert_available(BitsUtil::DOUBLE_SIZE_IN_BYTES, self.pos);
    let ret = BitsUtil::read_float64(&mut self.buffer, self.pos, self.big_endian);
    self.pos += BitsUtil::DOUBLE_SIZE_IN_BYTES as usize;
    ret
  }

  fn read_short_array(&mut self) -> Option<Vec<i16>> {
    self.read_array(Box::new(|this| this.read_short()))
  }

  fn read_char_array(&mut self) -> Option<Vec<char>> {
    self.read_array(Box::new(|this| this.read_char()))
  }

  fn read_float_array(&mut self) -> Option<Vec<f32>> {
    self.read_array(Box::new(|this| this.read_float()))
  }

  fn read_long_array(&mut self) -> Option<Vec<i64>> {
    self.read_array(Box::new(|this| this.read_long()))
  }

  fn read_double_array(&mut self) -> Option<Vec<f64>> {
    self.read_array(Box::new(|this| this.read_double()))
  }

  fn read_short_pos(&mut self, pos: usize) -> i16 {
    self.assert_available(BitsUtil::SHORT_SIZE_IN_BYTES, self.pos);
    BitsUtil::read_int16(&mut self.buffer, pos, self.big_endian)
  }

  fn assert_available(&self, num_of_bytes: i32, pos: usize) {
    assert!(pos >= 0);
    assert!(pos + num_of_bytes as usize <= self.buffer.len());
  }

  fn read_string(&mut self) -> Option<String> {
    let len = self.read_int();
    let read_pos = self.pos;
    if len == BitsUtil::NULL_ARRAY_LENGTH {
      return None;
    }
    let result = String::from_utf8(self.buffer[read_pos..read_pos + len as usize].to_vec()).unwrap();
    self.pos += len as usize;
    Some(result)
  }


  fn read_string_pos(&mut self, pos: usize) -> Option<String> {
    let len = self.read_int();
    let read_pos = pos + 4;
    if len == BitsUtil::NULL_ARRAY_LENGTH {
      return None;
    }
    let result = String::from_utf8(self.buffer[read_pos..read_pos + len as usize].to_vec()).unwrap();
    Some(result)
  }

  fn read_data(&mut self) -> Option<HeapData> {
    self.read_byte_array().map(HeapData::new)
  }

  fn read_string_array(&mut self) -> Option<Vec<String>> {
    self.read_array(Box::new(|this| this.read_string().unwrap()))
  }

  fn read_char(&mut self) -> char {
    self.assert_available(BitsUtil::CHAR_SIZE_IN_BYTES, self.pos);
    let read_bytes = BitsUtil::read_uint16(&mut self.buffer, self.pos, self.big_endian);
    self.pos += BitsUtil::CHAR_SIZE_IN_BYTES as usize;
    char::from_u32(read_bytes as u32).unwrap()
  }
}

impl DataInputReadObject for ObjectDataInput {
  fn read_object(&mut self) -> Box<dyn Any> {
    self.service.clone().read_object(self)
  }
}