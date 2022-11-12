use std::sync::Arc;
use crate::serialization::data::{DataOutput, DataOutputWriteObject, PositionalOutput};
use crate::serialization::heap_data::HeapData;
use crate::serialization::serializable::IdentifiedDataSerializableSerialization;
use crate::serialization::service::SerializationServiceV1;
use crate::util::bits_util::BitsUtil;

pub struct ObjectDataOutput {
  pub buffer: Vec<u8>,
  pub big_endian: bool,
  pub service: Arc<SerializationServiceV1>,
  pub pos: usize,
}

impl ObjectDataOutput {
  const OUTPUT_BUFFER_INITIAL_SIZE: i32 = HeapData::HEAP_DATA_OVERHEAD + BitsUtil::LONG_SIZE_IN_BYTES;
  const MASK_1BYTE: i32 = (1 << 8) - 1;

  pub fn new(big_endian: bool, service: Arc<SerializationServiceV1>) -> Self {
    ObjectDataOutput {
      buffer: vec![0; Self::OUTPUT_BUFFER_INITIAL_SIZE as usize],
      big_endian,
      service,
      pos: 0,
    }
  }
}

impl<T: IdentifiedDataSerializableSerialization> DataOutputWriteObject<T> for ObjectDataOutput {
  fn write_object(&mut self, object: &mut T) {
    self.service.clone().write_object(self, object);
  }
}

impl DataOutput for ObjectDataOutput {
  fn position(&self) -> usize {
    self.pos
  }

  fn clear(&mut self) {
    self.buffer.clear();
    self.pos = 0;
  }
  fn set_position(&mut self, new_position: usize) -> usize {
    let old_pos = self.pos;
    self.pos = new_position;
    old_pos
  }
  fn to_buffer(&self) -> Vec<u8> {
    self.buffer.clone()
  }

  fn write(&mut self, bytes: &Vec<u8>) {
    if bytes.len() == BitsUtil::BYTE_SIZE_IN_BYTES as usize {
      self.ensure_available(BitsUtil::BYTE_SIZE_IN_BYTES as usize);
      for i in 0..BitsUtil::BYTE_SIZE_IN_BYTES {
        BitsUtil::write_uint8(&mut self.buffer, self.pos + i as usize, ((bytes[i as usize] as i32) & Self::MASK_1BYTE) as u8);
        self.pos += BitsUtil::BYTE_SIZE_IN_BYTES as usize;
      }
    } else {
      self.ensure_available(bytes.len());
      self.buffer[self.pos..].copy_from_slice(bytes.as_slice());
      self.pos += bytes.len();
    }
  }

  fn write_byte(&mut self, byte: u8) {
    self.write(&vec![byte]);
  }

  fn write_boolean(&mut self, val: bool) {
    if val {
      self.write(&vec![1]);
    } else {
      self.write(&vec![0]);
    }
  }
  fn write_boolean_array(&mut self, bytes: Option<&Vec<bool>>) {
    self.write_array(|this, byte| this.write_boolean(*byte), bytes);
  }
  fn write_byte_array(&mut self, bytes: Option<&Vec<u8>>) {
    let len = if bytes.is_some() {
      bytes.unwrap().len()
    } else {
      BitsUtil::NULL_ARRAY_LENGTH as usize
    };
    if len > 0 {
      self.ensure_available(len);
      self.buffer[self.pos..].copy_from_slice(bytes.unwrap().as_slice());
      self.pos += len;
    }
  }
  fn write_char(&mut self, val: char) {
    self.ensure_available(BitsUtil::CHAR_SIZE_IN_BYTES as usize);
    BitsUtil::write_uint16(&mut self.buffer, self.pos as usize, val as u16, self.big_endian);
    self.pos += BitsUtil::CHAR_SIZE_IN_BYTES as usize;
  }
  fn write_char_array(&mut self, chars: Option<&Vec<char>>) {
    self.write_array(|this, el| this.write_char(*el), chars);
  }
  fn write_data(&mut self, data: Option<&HeapData>) {
    let buf = if data.is_some() {
      Some(data.unwrap().to_buffer())
    } else {
      None
    };

    let len = if buf.is_some() {
      buf.as_ref().unwrap().len()
    } else {
      BitsUtil::NULL_ARRAY_LENGTH as usize
    };

    self.write_int(len as i32);
    for bytes in buf.as_ref().unwrap() {
      self.write(&vec![*bytes]);
    }
  }
  fn write_double(&mut self, double: f64) {
    self.ensure_available(BitsUtil::DOUBLE_SIZE_IN_BYTES as usize);
    BitsUtil::write_float64(&mut self.buffer, self.pos, double, self.big_endian);
    self.pos += BitsUtil::DOUBLE_SIZE_IN_BYTES as usize;
  }
  fn write_double_array(&mut self, doubles: Option<&Vec<f64>>) {
    self.write_array(|this, el| this.write_double(*el), doubles);
  }
  fn write_float(&mut self, float: f32) {
    self.ensure_available(BitsUtil::FLOAT_SIZE_IN_BYTES as usize);
    BitsUtil::write_float32(&mut self.buffer, self.pos, float, self.big_endian);
    self.pos += BitsUtil::FLOAT_SIZE_IN_BYTES as usize;
  }
  fn write_float_array(&mut self, floats: Option<&Vec<f32>>) {
    self.write_array(|this, el| this.write_float(*el), floats);
  }

  fn write_int8(&mut self, val: i8) {
    self.ensure_available(BitsUtil::BYTE_SIZE_IN_BYTES as usize);
    BitsUtil::write_int8(&mut self.buffer, self.pos, val);
    self.pos += BitsUtil::BYTE_SIZE_IN_BYTES as usize;
  }

  fn write_int(&mut self, number: i32) {
    self.ensure_available(BitsUtil::INT_SIZE_IN_BYTES as usize);
    BitsUtil::write_int32(&mut self.buffer, self.pos as usize, number, self.big_endian);
    self.pos += BitsUtil::BYTE_SIZE_IN_BYTES as usize;
  }
  fn write_int_be(&mut self, number: i32) {
    self.ensure_available(BitsUtil::INT_SIZE_IN_BYTES as usize);
    BitsUtil::write_int32(&mut self.buffer, self.pos as usize, number, true);
    self.pos += BitsUtil::BYTE_SIZE_IN_BYTES as usize;
  }
  fn write_int_array(&mut self, ints: Option<&Vec<i32>>) {
    self.write_array(|this, el| this.write_int(*el), ints);
  }
  fn write_long(&mut self, number: i64) {
    self.ensure_available(BitsUtil::LONG_SIZE_IN_BYTES as usize);
    if self.big_endian {
      BitsUtil::write_int64(&mut self.buffer, self.pos as usize, number, true);
      self.pos += BitsUtil::LONG_SIZE_IN_BYTES as usize;
    } else {
      BitsUtil::write_int64(&mut self.buffer, self.pos as usize, number, false);
      self.pos += BitsUtil::LONG_SIZE_IN_BYTES as usize;
    }
  }
  fn write_long_array(&mut self, longs: Option<&Vec<i64>>) {
    self.write_array(|this, el| this.write_long(*el), longs);
  }

  fn write_short(&mut self, short: i16) {
    self.ensure_available(BitsUtil::SHORT_SIZE_IN_BYTES as usize);
    BitsUtil::write_int16(&mut self.buffer, self.pos as usize, short, self.big_endian);
    self.pos += BitsUtil::SHORT_SIZE_IN_BYTES as usize;
  }
  fn write_short_array(&mut self, shorts: Option<&Vec<i16>>) {
    self.write_array(|this, el| this.write_short(*el), shorts);
  }
  fn write_utf(&mut self, val: Option<&String>) {
    self.write_string(val);
  }
  fn write_string(&mut self, val: Option<&String>) {
    let len = if val.is_some() {
      val.as_ref().unwrap().as_bytes().len()
    } else {
      BitsUtil::NULL_ARRAY_LENGTH as usize
    };
    self.write_int(len as i32);
    if len == BitsUtil::NULL_ARRAY_LENGTH as usize {
      return;
    }

    self.ensure_available(len);
    self.buffer[self.pos..self.pos + len].copy_from_slice(val.unwrap().as_bytes());
    self.pos += len;
  }
  fn write_utf_array(&mut self, val: Option<&Vec<String>>) {
    self.write_array(|this, el| this.write_utf(Some(&el)), val);
  }
  fn write_string_array(&mut self, val: Option<&Vec<String>>) {
    self.write_array(|this, el| this.write_string(Some(&el)), val);
  }
  fn write_zero_bytes(&mut self, count: i32) {
    for _ in 0..count {
      self.write(&vec![0]);
    }
  }
  fn available(&self) -> i32 {
    (self.buffer.len() - self.pos) as i32
  }
  fn ensure_available(&mut self, size: usize) {
    if self.available() < size as i32 {
      let mut new_buffer = vec![0; (self.pos + size) as usize];
      new_buffer[..self.pos].copy_from_slice(&self.buffer[..self.pos]);
      self.buffer = new_buffer;
    }
  }
  fn write_array<T>(&mut self, func: impl Fn(&mut Self, &T), arr: Option<&Vec<T>>) {
    let len = if arr.is_some() {
      arr.unwrap().len()
    } else {
      BitsUtil::NULL_ARRAY_LENGTH as usize
    };
    self.write_int(len as i32);
    if len > 0 {
      for el in arr.unwrap() {
        func(self, el);
      }
    }
  }
}

impl PositionalOutput for ObjectDataOutput {
  fn pwrite(&mut self, position: usize, bytes: Vec<u8>) {
    todo!()
  }

  fn pwrite_boolean(&mut self, position: usize, val: bool) {
    todo!()
  }

  fn pwrite_byte(&mut self, position: usize, byte: u8) {
    todo!()
  }

  fn pwrite_int8(&mut self, position: usize, byte: i8) {
    todo!()
  }

  fn pwrite_char(&mut self, position: usize, val: char) {
    todo!()
  }

  fn pwrite_double(&mut self, position: usize, double: f64) {
    todo!()
  }

  fn pwrite_float(&mut self, position: usize, float: f32) {
    todo!()
  }

  fn pwrite_int(&mut self, position: usize, number: i32) {
    todo!()
  }

  fn pwrite_long(&mut self, position: usize, number: i64) {
    todo!()
  }

  fn pwrite_int_be(&mut self, position: usize, number: i32) {
    todo!()
  }

  fn pwrite_short(&mut self, position: usize, short: i16) {
    todo!()
  }

  fn pwrite_boolean_bit(&mut self, position: usize, bit: usize, val: bool) {
    todo!()
  }
}