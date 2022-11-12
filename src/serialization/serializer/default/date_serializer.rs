use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime};
use crate::serialization::data::{DataInput, DataOutput};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::serializer::Serializer;

#[derive(Default)]
pub struct DateSerializer;

impl Serializer<Box<NaiveDate>> for DateSerializer {
  fn id(&self) -> i32 {
    -25
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<NaiveDate> {
    NaiveDateTime::from_timestamp(input.read_long(), 0).date().into()
  }

  fn write(&self, output: &mut ObjectDataOutput, object: Box<NaiveDate>) {
    output.write_long(object.and_time(NaiveTime::default()).timestamp_millis());
  }
}