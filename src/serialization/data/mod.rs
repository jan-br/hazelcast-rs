pub mod object_data_input;
pub mod object_data_output;

use std::any::Any;
use crate::serialization::heap_data::HeapData;
use crate::serialization::serializable::{IdentifiedDataSerializable, IdentifiedDataSerializableSerialization};

pub trait DataOutputWriteObject<T: IdentifiedDataSerializableSerialization> {
  fn write_object(&mut self, object: &mut T);
}

pub trait DataOutput {
  fn position(&self) -> usize;
  fn clear(&mut self);
  fn set_position(&mut self, new_position: usize) -> usize;
  fn to_buffer(&self) -> Vec<u8>;
  fn write(&mut self, bytes: &Vec<u8>);
  fn write_byte(&mut self, byte: u8);
  fn write_boolean(&mut self, val: bool);
  fn write_boolean_array(&mut self, bytes: Option<&Vec<bool>>);
  fn write_byte_array(&mut self, bytes: Option<&Vec<u8>>);
  fn write_char(&mut self, val: char);
  fn write_char_array(&mut self, chars: Option<&Vec<char>>);
  fn write_data(&mut self, data: Option<&HeapData>);
  fn write_double(&mut self, double: f64);
  fn write_double_array(&mut self, doubles: Option<&Vec<f64>>);
  fn write_float(&mut self, float: f32);
  fn write_float_array(&mut self, floats: Option<&Vec<f32>>);
  fn write_int8(&mut self, val: i8);
  fn write_int(&mut self, number: i32);
  fn write_int_be(&mut self, number: i32);
  fn write_int_array(&mut self, ints: Option<&Vec<i32>>);
  fn write_long(&mut self, number: i64);
  fn write_long_array(&mut self, longs: Option<&Vec<i64>>);
  fn write_short(&mut self, short: i16);
  fn write_short_array(&mut self, shorts: Option<&Vec<i16>>);
  fn write_utf(&mut self, val: Option<&String>);
  fn write_string(&mut self, val: Option<&String>);
  fn write_utf_array(&mut self, val: Option<&Vec<String>>);
  fn write_string_array(&mut self, val: Option<&Vec<String>>);
  fn write_zero_bytes(&mut self, count: i32);
  fn available(&self) -> i32;
  fn ensure_available(&mut self, size: usize);
  fn write_array<T>(&mut self, func: impl Fn(&mut Self, &T), arr: Option<&Vec<T>>);
}

pub trait DataInputReadObject {
  fn read_object(&mut self) -> Box<dyn Any>;
}

pub trait DataInput {
  fn position(&self) -> usize;
  fn set_position(&mut self, new_position: usize) -> usize;
  fn read(&mut self) -> u8;
  fn read_pos(&mut self, pos: usize) -> u8;
  fn read_byte(&mut self) -> u8;
  fn read_byte_pos(&mut self, pos: usize) -> u8;
  fn read_boolean(&mut self) -> bool;
  fn read_boolean_pos(&mut self, pos: usize) -> bool;
  fn read_boolean_array(&mut self) -> Option<Vec<bool>>;
  fn read_boolean_array_pos(&mut self, pos: usize) -> Option<Vec<bool>>;
  fn read_byte_array(&mut self) -> Option<Vec<u8>>;
  fn read_byte_array_pos(&mut self, pos: usize) -> Option<Vec<u8>>;
  fn read_array<T>(&mut self, func: Box<dyn Fn(&mut Self) -> T>) -> Option<Vec<T>>;
  fn read_array_pos<T>(&mut self, func: Box<dyn Fn(&mut Self) -> T>, pos: usize) -> Option<Vec<T>>;
  fn read_int_array(&mut self) -> Option<Vec<i32>>;
  fn read_int(&mut self) -> i32;
  fn read_long(&mut self) -> i64;
  fn read_int_pos(&mut self, pos: usize) -> i32;
  fn read_float(&mut self) -> f32;
  fn assert_available(&self, num_of_bytes: i32, pos: usize);
  fn read_short(&mut self) -> i16;
  fn read_short_pos(&mut self, pos: usize) -> i16;
  fn read_string(&mut self) -> Option<String>;
  fn read_string_pos(&mut self, pos: usize) -> Option<String>;
  fn read_data(&mut self) -> Option<HeapData>;
  fn read_double(&mut self) -> f64;
  fn read_short_array(&mut self) -> Option<Vec<i16>>;
  fn read_long_array(&mut self) -> Option<Vec<i64>>;
  fn read_double_array(&mut self) -> Option<Vec<f64>>;
  fn read_string_array(&mut self) -> Option<Vec<String>>;
  fn read_char(&mut self) -> char;
  fn read_char_array(&mut self) -> Option<Vec<char>>;
  fn read_float_array(&mut self) -> Option<Vec<f32>>;
}

pub trait PositionalOutput {
  fn pwrite(&mut self, position: usize, bytes: Vec<u8>);
  fn pwrite_boolean(&mut self, position: usize, val: bool);
  fn pwrite_byte(&mut self, position: usize, byte: u8);
  fn pwrite_int8(&mut self, position: usize, byte: i8);
  fn pwrite_char(&mut self, position: usize, val: char);
  fn pwrite_double(&mut self, position: usize, double: f64);
  fn pwrite_float(&mut self, position: usize, float: f32);
  fn pwrite_int(&mut self, position: usize, number: i32);
  fn pwrite_long(&mut self, position: usize, number: i64);
  fn pwrite_int_be(&mut self, position: usize, number: i32);
  fn pwrite_short(&mut self, position: usize, short: i16);
  fn pwrite_boolean_bit(&mut self, position: usize, bit: usize, val: bool);
}
