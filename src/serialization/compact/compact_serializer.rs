use std::any::{Any, TypeId};
use crate::serialization::compact::compact_writer::CompactWriter;
use crate::serialization::compact::compact_reader::CompactReader;

pub trait CompactSerializer<T> : Send + Sync {
  fn get_type(&self) -> TypeId;
  fn get_type_name(&self) -> String;
  fn read(&self, reader: &mut dyn CompactReader) -> T;
  fn write(&self, writer: &mut dyn CompactWriter, obj: &T);
}