use byteorder::*;
use bytes::{Buf, BufMut};

pub struct BitsUtil;

impl BitsUtil {
  pub const BYTE_SIZE_IN_BYTES: i32 = 1;
  pub const BOOLEAN_SIZE_IN_BYTES: i32 = 1;
  pub const SHORT_SIZE_IN_BYTES: i32 = 2;
  pub const CHAR_SIZE_IN_BYTES: i32 = 2;
  pub const INT_SIZE_IN_BYTES: i32 = 4;
  pub const FLOAT_SIZE_IN_BYTES: i32 = 4;
  pub const LONG_SIZE_IN_BYTES: i32 = 8;
  pub const DOUBLE_SIZE_IN_BYTES: i32 = 8;
  pub const LOCAL_DATE_SIZE_IN_BYTES: i32 = Self::INT_SIZE_IN_BYTES + Self::BYTE_SIZE_IN_BYTES * 2;
  pub const LOCAL_TIME_SIZE_IN_BYTES: i32 = Self::INT_SIZE_IN_BYTES + Self::BYTE_SIZE_IN_BYTES * 3;
  pub const LOCAL_DATETIME_SIZE_IN_BYTES: i32 = Self::LOCAL_DATE_SIZE_IN_BYTES + Self::LOCAL_TIME_SIZE_IN_BYTES;
  pub const OFFSET_DATE_TIME_SIZE_IN_BYTES: i32 = Self::LOCAL_DATETIME_SIZE_IN_BYTES + Self::INT_SIZE_IN_BYTES;

  pub const UUID_SIZE_IN_BYTES: i32 = Self::BOOLEAN_SIZE_IN_BYTES + 2 * Self::LONG_SIZE_IN_BYTES;

  pub const NULL_ARRAY_LENGTH: i32 = -1;
  pub const BITS_IN_A_BYTE: i32 = 8;

  pub fn write_uint32(buffer: &mut Vec<u8>, pos: usize, val: u32, is_big_endian: bool) {
    if is_big_endian {
      buffer[pos..pos + 4].writer().write_u32::<BigEndian>(val).unwrap();
    } else {
      buffer[pos..pos + 4].writer().write_u32::<LittleEndian>(val).unwrap();
    }
  }

  pub fn write_uint16(buffer: &mut Vec<u8>, pos: usize, val: u16, is_big_endian: bool) {
    if is_big_endian {
      buffer[pos..pos + 2].writer().write_u16::<BigEndian>(val as u16).unwrap();
    } else {
      buffer[pos..pos + 2].writer().write_u16::<LittleEndian>(val as u16).unwrap();
    }
  }

  pub fn write_uint8(buffer: &mut Vec<u8>, pos: usize, val: u8) {
    buffer[pos] = val;
  }

  pub fn write_int64(buffer: &mut Vec<u8>, pos: usize, val: i64, is_big_endian: bool) {
    if is_big_endian {
      buffer[pos..pos + 8].writer().write_i64::<BigEndian>(val).unwrap();
    } else {
      buffer[pos..pos + 8].writer().write_i64::<LittleEndian>(val).unwrap();
    }
  }

  pub fn write_int32(buffer: &mut Vec<u8>, pos: usize, val: i32, is_big_endian: bool) {
    if is_big_endian {
      buffer[pos..pos + 4].writer().write_i32::<BigEndian>(val).unwrap();
    } else {
      buffer[pos..pos + 4].writer().write_i32::<LittleEndian>(val).unwrap();
    }
  }

  pub fn write_int16(buffer: &mut Vec<u8>, pos: usize, val: i16, is_big_endian: bool) {
    if is_big_endian {
      buffer[pos..pos + 2].writer().write_i16::<BigEndian>(val).unwrap();
    } else {
      buffer[pos..pos + 2].writer().write_i16::<LittleEndian>(val).unwrap();
    }
  }

  pub fn write_int8(buffer: &mut Vec<u8>, pos: usize, val: i8) {
    buffer[pos] = val as u8;
  }

  pub fn write_float32(buffer: &mut Vec<u8>, pos: usize, val: f32, is_big_endian: bool) {
    if is_big_endian {
      buffer[pos..pos + 4].writer().write_f32::<BigEndian>(val).unwrap();
    } else {
      buffer[pos..pos + 4].writer().write_f32::<LittleEndian>(val).unwrap();
    }
  }

  pub fn write_float64(buffer: &mut Vec<u8>, pos: usize, val: f64, is_big_endian: bool) {
    if is_big_endian {
      buffer[pos..pos + 8].writer().write_f64::<BigEndian>(val).unwrap();
    } else {
      buffer[pos..pos + 8].writer().write_f64::<LittleEndian>(val).unwrap();
    }
  }

  pub fn write_boolean(buffer: &mut Vec<u8>, pos: usize, val: bool) {
    buffer[pos] = if val { 1 } else { 0 };
  }

  pub fn read_uint64(buffer: &[u8], pos: usize, is_big_endian: bool) -> u64 {
    if is_big_endian {
      buffer[pos..pos + 8].reader().read_u64::<BigEndian>().unwrap()
    } else {
      buffer[pos..pos + 8].reader().read_u64::<LittleEndian>().unwrap()
    }
  }

  pub fn read_uint32(buffer: &[u8], pos: usize, is_big_endian: bool) -> u32 {
    if is_big_endian {
      buffer[pos..pos + 4].reader().read_u32::<BigEndian>().unwrap()
    } else {
      buffer[pos..pos + 4].reader().read_u32::<LittleEndian>().unwrap()
    }
  }

  pub fn read_uint16(buffer: &[u8], pos: usize, is_big_endian: bool) -> u16 {
    if is_big_endian {
      buffer[pos..pos + 2].reader().read_u16::<BigEndian>().unwrap()
    } else {
      buffer[pos..pos + 2].reader().read_u16::<LittleEndian>().unwrap()
    }
  }

  pub fn read_uint8(buffer: &[u8], pos: usize) -> u8 {
    buffer[pos]
  }

  pub fn read_int64(buffer: &[u8], pos: usize, is_big_endian: bool) -> i64 {
    if is_big_endian {
      buffer[pos..pos + 8].reader().read_i64::<BigEndian>().unwrap()
    } else {
      buffer[pos..pos + 8].reader().read_i64::<LittleEndian>().unwrap()
    }
  }

  pub fn read_int32(buffer: &[u8], pos: usize, is_big_endian: bool) -> i32 {
    if is_big_endian {
      buffer[pos..pos + 4].reader().read_i32::<BigEndian>().unwrap()
    } else {
      buffer[pos..pos + 4].reader().read_i32::<LittleEndian>().unwrap()
    }
  }

  pub fn read_int16(buffer: &[u8], pos: usize, is_big_endian: bool) -> i16 {
    if is_big_endian {
      buffer[pos..pos + 2].reader().read_i16::<BigEndian>().unwrap()
    } else {
      buffer[pos..pos + 2].reader().read_i16::<LittleEndian>().unwrap()
    }
  }

  pub fn read_int8(buffer: &[u8], pos: usize) -> i8 {
    buffer[pos] as i8
  }

  pub fn read_float32(buffer: &[u8], pos: usize, is_big_endian: bool) -> f32 {
    if is_big_endian {
      buffer[pos..pos + 4].reader().read_f32::<BigEndian>().unwrap()
    } else {
      buffer[pos..pos + 4].reader().read_f32::<LittleEndian>().unwrap()
    }
  }

  pub fn read_float64(buffer: &[u8], pos: usize, is_big_endian: bool) -> f64 {
    if is_big_endian {
      buffer[pos..pos + 8].reader().read_f64::<BigEndian>().unwrap()
    } else {
      buffer[pos..pos + 8].reader().read_f64::<LittleEndian>().unwrap()
    }
  }

  pub fn read_boolean(buffer: &[u8], pos: usize) -> bool {
    buffer[pos] != 0
  }
}
