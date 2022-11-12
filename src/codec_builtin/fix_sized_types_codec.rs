use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};
use uuid::Uuid;
use crate::util::bits_util::BitsUtil;

const TWO_PWR_16_DBL: u128 = 1 << 16;
const TWO_PWR_32_DBL: u128 = TWO_PWR_16_DBL * TWO_PWR_16_DBL;
const TWO_PWR_64_DBL: u128 = TWO_PWR_32_DBL * TWO_PWR_32_DBL;
const TWO_PWR_63_DBL: u128 = TWO_PWR_64_DBL / 2;

pub struct FixSizedTypesCodec;

impl FixSizedTypesCodec {
  pub async fn encode_int(buffer: &mut Vec<u8>, offset: usize, value: &i32) {
    BitsUtil::write_int32(buffer, offset, *value, false);
  }

  pub async fn decode_int(buffer: &Vec<u8>, offset: usize) -> i32 {
    BitsUtil::read_int32(buffer, offset, false)
  }

  pub async fn decode_local_date(buffer: &Vec<u8>, offset: usize) -> NaiveDate {
      let year = Self::decode_int(buffer, offset).await;
      let month = Self::decode_byte(buffer, offset + BitsUtil::INT_SIZE_IN_BYTES as usize).await;
      let date = Self::decode_byte(buffer, offset + BitsUtil::INT_SIZE_IN_BYTES as usize + BitsUtil::BYTE_SIZE_IN_BYTES as usize).await;
    NaiveDate::from_ymd(year, month as u32, date as u32)
  }

  pub async fn decode_local_date_time(buffer: &Vec<u8>, offset: usize) -> NaiveDateTime {
      let date = Self::decode_local_date(buffer, offset).await;
      let time = Self::decode_local_time(buffer, offset + BitsUtil::LOCAL_DATE_SIZE_IN_BYTES as usize).await;
    NaiveDateTime::new(date, time)
  }

  pub async fn decode_offset_date_time(buffer: &Vec<u8>, offset: usize) -> DateTime<FixedOffset> {
  let local_date_time = Self::decode_local_date_time(buffer, offset).await;
  let offset_seconds = Self::decode_int(buffer, offset + BitsUtil::LOCAL_DATETIME_SIZE_IN_BYTES as usize).await;
    //todo: check if this is correct
    DateTime::from_utc(local_date_time, FixedOffset::east(offset_seconds))
  }

  pub async fn decode_local_time(buffer: &Vec<u8>, offset: usize) -> NaiveTime {
      let hour = Self::decode_byte(buffer, offset).await;
      let minute = Self::decode_byte(buffer, offset + BitsUtil::BYTE_SIZE_IN_BYTES as usize).await;
      let second = Self::decode_byte(buffer, offset + BitsUtil::BYTE_SIZE_IN_BYTES as usize * 2).await;
      let nano = Self::decode_int(buffer, offset + BitsUtil::BYTE_SIZE_IN_BYTES as usize * 3).await;
    NaiveTime::from_hms_nano(hour as u32, minute as u32, second as u32, nano as u32)
  }

  pub async fn decode_short(buffer: &Vec<u8>, offset: usize) -> i16 {
      BitsUtil::read_int16(buffer, offset, false)
  }

  pub async fn decode_float(buffer: &Vec<u8>, offset: usize) -> f32 {
      BitsUtil::read_float32(buffer, offset, false)
  }

  pub async fn decode_double(buffer: &mut Vec<u8>, offset: usize) -> f64 {
      BitsUtil::read_float64(buffer, offset, false)
  }

  pub async fn encode_long(buffer: &mut Vec<u8>, offset: usize, value: &i64) {
      BitsUtil::write_int64(buffer, offset, *value, false);
  }

  pub async fn decode_long(buffer: &Vec<u8>, offset: usize) -> i64 {
      BitsUtil::read_int64(buffer, offset, false)
  }

  pub async fn encode_non_negative_number_as_long(buffer: &mut Vec<u8>, offset: usize, value: u64) {
    if value < 0 {
      panic!("value must be non-negative");
    }

    if (value + 1) as u128 >= TWO_PWR_63_DBL {
      //todo: check if this is correct
        BitsUtil::write_int32(buffer, offset, -1_i32, false);
      BitsUtil::write_int32(buffer, offset + BitsUtil::INT_SIZE_IN_BYTES as usize, 0x7FFFFFFF, false);
      return;
    }

    BitsUtil::write_int32(buffer, offset, (value as u128 % TWO_PWR_32_DBL) as i32, false);
    BitsUtil::write_int32(buffer, offset + BitsUtil::INT_SIZE_IN_BYTES as usize, (value as u128 / TWO_PWR_32_DBL) as i32, false);
  }

  pub async fn decode_number_from_long(buffer: &Vec<u8>, offset: usize) -> u64 {
    let low = BitsUtil::read_int32(buffer, offset, false);
    let high = BitsUtil::read_int32(buffer, offset + BitsUtil::INT_SIZE_IN_BYTES as usize, false);
    (high as i64 * TWO_PWR_32_DBL as i64 + low as i64) as u64
  }

  pub async fn encode_boolean(buffer: &mut Vec<u8>, offset: usize, value: &bool) {
    BitsUtil::write_boolean(buffer, offset, *value);
  }

  pub async fn decode_boolean(buffer: &Vec<u8>, offset: usize) -> bool {
    BitsUtil::read_boolean(buffer, offset)
  }

  pub async fn encode_byte(buffer: &mut Vec<u8>, offset: usize, value: &u8) {
    BitsUtil::write_uint8(buffer, offset, *value);
  }

  pub async fn decode_byte(buffer: &Vec<u8>, offset: usize) -> u8 {
    BitsUtil::read_uint8(buffer, offset)
  }

  pub async fn encode_uuid(buffer: &mut Vec<u8>, offset: usize, value: &Uuid) {
    Self::encode_uuid_nullable(buffer, offset, &Some(value)).await;
  }

  pub async fn encode_uuid_nullable(buffer: &mut Vec<u8>, offset: usize, value: &Option<&Uuid>) {
    let is_null = value.is_none();
    Self::encode_boolean(buffer, offset, &is_null).await;
    if is_null {
      return;
    }
    let most_significant_bits = (value.unwrap().as_u128() >> 64) & 0xFFFFFFFFFFFFFFFF;
    let least_significant_bits = value.unwrap().as_u128() & 0xFFFFFFFFFFFFFFFF;
    Self::encode_long(buffer, offset + BitsUtil::BOOLEAN_SIZE_IN_BYTES as usize, &(most_significant_bits as i64)).await;
    Self::encode_long(buffer, offset + BitsUtil::BOOLEAN_SIZE_IN_BYTES as usize + BitsUtil::LONG_SIZE_IN_BYTES as usize, &(least_significant_bits as i64)).await;
  }

  pub async fn decode_uuid(buffer: &Vec<u8>, offset: usize) -> Uuid {
      Self::decode_uuid_nullable(buffer, offset).await.unwrap()
  }

  pub async fn decode_uuid_nullable(buffer: &Vec<u8>, offset: usize) -> Option<Uuid> {
      let is_null = Self::decode_boolean(buffer, offset).await;
    if is_null {
      return None;
    }

      let most_significant_bits = Self::decode_long(buffer, offset + BitsUtil::BOOLEAN_SIZE_IN_BYTES as usize).await;
    let least_significant_bits = Self::decode_long(buffer, offset + BitsUtil::BOOLEAN_SIZE_IN_BYTES as usize + BitsUtil::LONG_SIZE_IN_BYTES as usize).await;
    Some(Uuid::from_u64_pair(most_significant_bits as u64, least_significant_bits as u64))
  }
}