use crate::core::big_decimal::BigDecimal;
use crate::serialization::data::{DataInput, DataOutput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;
use crate::util::big_decimal_util::BigDecimalUtil;

#[derive(Default)]
pub struct BigDecimalSerializer;

impl Serializer<Box<BigDecimal>> for BigDecimalSerializer {
  fn id(&self) -> i32 {
    -27
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<BigDecimal> {
    let body = input.read_byte_array().unwrap();
    let scale = input.read_int();

    BigDecimal::new(BigDecimalUtil::buffer_to_big_int(&body), scale).into()
  }

  fn write(&self, output: &mut ObjectDataOutput, object: Box<BigDecimal>) {
    output.write_byte_array(Some(&BigDecimalUtil::big_int_to_buffer(&object.unscaled_value)));
    output.write_int(object.scale);
  }
}