use num_bigint::BigInt;
use crate::serialization::data::{DataInput, DataOutput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;
use crate::util::big_decimal_util::BigDecimalUtil;

#[derive(Default)]
pub struct BigIntSerializer;

impl Serializer<Box<BigInt>> for BigIntSerializer {
  fn id(&self) -> i32 {
    -26
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<BigInt> {
    let body = input.read_byte_array().unwrap();
    BigDecimalUtil::buffer_to_big_int(&body).into()
  }

  fn write(&self, output: &mut ObjectDataOutput, object: Box<BigInt>) {
    output.write_byte_array(Some(&BigDecimalUtil::big_int_to_buffer(&object)));
  }
}