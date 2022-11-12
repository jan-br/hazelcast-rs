use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::DataInput;
use crate::util::bits_util::BitsUtil;

pub struct OffsetConstants;

impl OffsetConstants {
  pub const BYTE_MAX_VALUE: i32 = 127;
  pub const BYTE_MIN_VALUE: i32 = -128;
  pub const SHORT_MAX_VALUE: i32 = 32767;
  pub const SHORT_MIN_VALUE: i32 = -32768;
  pub const BYTE_OFFSET_READER_RANGE: i32 = Self::BYTE_MAX_VALUE - Self::BYTE_MIN_VALUE;
  pub const NULL_OFFSET: i32 = -1;
  pub const SHORT_OFFSET_READER_RANGE: i32 = Self::SHORT_MAX_VALUE - Self::SHORT_MIN_VALUE;

  pub fn read_byte_offset(
    input: &mut ObjectDataInput,
    variable_offsets_pos: i32,
    index: i32,
  ) -> i32 {
    let offset = input.read_byte_pos((variable_offsets_pos + index) as usize) as i32;
    if offset == (Self::NULL_OFFSET & 0xFF) {
      return Self::NULL_OFFSET;
    }
    offset
  }

  pub fn read_short_offset(
    input: &mut ObjectDataInput,
    variable_offsets_pos: i32,
    index: i32,
  ) -> i32 {
    let offset =
        input.read_short_pos((variable_offsets_pos + (index * BitsUtil::SHORT_SIZE_IN_BYTES)) as usize);
    if offset == Self::NULL_OFFSET as i16 {
      Self::NULL_OFFSET as i32
    } else {
      offset as i32 & 0xFFFF
    }
  }

  pub fn read_int_offset(input: &mut ObjectDataInput, variable_offsets_pos: i32, index: i32) -> i32 {
    input.read_int_pos((variable_offsets_pos + (index * BitsUtil::INT_SIZE_IN_BYTES)) as usize)
  }
}
