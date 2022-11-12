use std::mem::transmute;
use crate::serialization::generic_record::field_kind_based_operations::FieldKindBasedOperations;
use crate::serialization::generic_record::field_operations::BooleanFieldKindBasedOperations;

#[repr(i32)]
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum FieldKind {
  Boolean = 0,
  ArrayOfBoolean = 1,
  Int8 = 2,
  ArrayOfInt8 = 3,

  Int16 = 6,
  ArrayOfInt16 = 7,
  Int32 = 8,
  ArrayOfInt32 = 9,
  Int64 = 10,
  ArrayOfInt64 = 11,
  Float32 = 12,
  ArrayOfFloat32 = 13,
  Float64 = 14,
  ArrayOfFloat64 = 15,
  String = 16,
  ArrayOfString = 17,
  Decimal = 18,
  ArrayOfDecimal = 19,
  Time = 20,
  ArrayOfTime = 21,
  Date = 22,
  ArrayOfDate = 23,
  Timestamp = 24,
  ArrayOfTimestamp = 25,
  TimestampWithTimezone = 26,
  ArrayOfTimestampWithTimezone = 27,
  Compact = 28,
  ArrayOfCompact = 29,

  NullableBoolean = 32,
  ArrayOfNullableBoolean = 33,
  NullableInt8 = 34,
  ArrayOfNullableInt8 = 35,
  NullableInt16 = 36,
  ArrayOfNullableInt16 = 37,
  NullableInt32 = 38,
  ArrayOfNullableInt32 = 39,
  NullableInt64 = 40,
  ArrayOfNullableInt64 = 41,
  NullableFloat32 = 42,
  ArrayOfNullableFloat32 = 43,
  NullableFloat64 = 44,
  ArrayOfNullableFloat64 = 45,
  NotAvailable = 46,
}

impl From<i32> for FieldKind {
  fn from(value: i32) -> Self {
    unsafe { transmute(value) }
  }
}

impl FieldKind {
  pub fn get_operation(&self) -> Box<dyn FieldKindBasedOperations> {
    match self {
      FieldKind::Boolean => Box::new(BooleanFieldKindBasedOperations),
      _ => todo!()
    }
  }
}