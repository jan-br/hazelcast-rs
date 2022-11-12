use std::pin::Pin;
use std::sync::Arc;

use crate::serialization::compact::compact_stream_serializer::CompactStreamSerializer;
use crate::serialization::compact::offset_reader::OffsetReader;
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::DataInput;
use crate::serialization::schema::Schema;
use crate::util::bits_util::BitsUtil;

use super::compact_reader::CompactReader;
use super::offset_constants::OffsetConstants;

pub struct DefaultCompactReader<'a, 'b> {
  pub offset_reader: Pin<Box<OffsetReader>>,
  pub variable_offsets_position: usize,
  pub data_start_position: usize,
  pub serializer: &'b CompactStreamSerializer,
  pub input: &'a mut ObjectDataInput,
  pub schema: Arc<Schema>,
}

impl<'a, 'b> DefaultCompactReader<'a, 'b> {
  pub fn new(
    serializer: &'b CompactStreamSerializer,
    input: &'a mut ObjectDataInput,
    schema: Arc<Schema>,
  ) -> Self {
    let number_of_variable_length_fields = schema.number_var_size_fields;
    let mut final_position = 0;
    let mut data_start_position = 0;
    let mut variable_offsets_position = 0;
    let mut offset_reader: Pin<Box<dyn Fn(&mut ObjectDataInput, i32, i32) -> i32>> = Box::pin(OffsetConstants::read_byte_offset);

    if number_of_variable_length_fields != 0 {
      let data_length = input.read_int();
      data_start_position = input.position();
      variable_offsets_position = data_start_position + data_length as usize;
      if data_length < OffsetConstants::BYTE_OFFSET_READER_RANGE {
        offset_reader = Box::pin(OffsetConstants::read_byte_offset);
        final_position = variable_offsets_position + number_of_variable_length_fields
      } else if data_length < OffsetConstants::SHORT_OFFSET_READER_RANGE {
        offset_reader = Box::pin(OffsetConstants::read_short_offset);
        final_position = variable_offsets_position
            + number_of_variable_length_fields * BitsUtil::SHORT_SIZE_IN_BYTES as usize;
      } else {
        offset_reader = Box::pin(OffsetConstants::read_int_offset);
        final_position = variable_offsets_position
            + number_of_variable_length_fields * BitsUtil::INT_SIZE_IN_BYTES as usize;
      }
    } else {
      offset_reader = Box::pin(OffsetConstants::read_int_offset);
      variable_offsets_position = 0;
      data_start_position = input.position();
      final_position = data_start_position + schema.fixed_size_fields_length;
    }
    input.set_position(final_position);
    Self {
      offset_reader,
      variable_offsets_position,
      data_start_position,
      serializer,
      input,
      schema,
    }
  }
}

impl<'a, 'b> CompactReader for DefaultCompactReader<'a, 'b> {}