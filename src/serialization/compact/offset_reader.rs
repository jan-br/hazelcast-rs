use crate::serialization::data::object_data_input::ObjectDataInput;

pub type OffsetReader = dyn Fn(&mut ObjectDataInput, i32, i32) -> i32;