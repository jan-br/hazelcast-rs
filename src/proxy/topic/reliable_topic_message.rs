use std::any::Any;
use std::sync::Arc;
use crate::connection::address::Address;
use crate::serialization::data::{DataInput, DataInputReadObject, DataOutput, DataOutputWriteObject};
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::heap_data::HeapData;
use crate::serialization::serializable::{IdentifiedDataSerializable, IdentifiedDataSerializableSerialization, IdentifiedDataSerializableInfo};


pub const RELIABLE_TOPIC_MESSAGE_FACTORY_ID: i32 = -9;
pub const RELIABLE_TOPIC_CLASS_ID: i32 = 2;

pub struct ReliableTopicMessage {
  pub factory_id: i32,
  pub class_id: i32,
  pub publish_time: Option<i64>,
  pub publisher_address: Option<Box<Address>>,
  pub payload: Option<HeapData>,
}

impl ReliableTopicMessage {
  pub fn new() -> Self {
    Self {
      factory_id: RELIABLE_TOPIC_MESSAGE_FACTORY_ID,
      class_id: RELIABLE_TOPIC_CLASS_ID,
      publish_time: None,
      publisher_address: None,
      payload: None,
    }
  }
}

impl IdentifiedDataSerializableInfo for ReliableTopicMessage {
  fn factory_id(&self) -> i32 {
    self.factory_id
  }

  fn class_id(&self) -> i32 {
    self.class_id
  }
}

impl IdentifiedDataSerializableSerialization for ReliableTopicMessage {
  fn read_data(&mut self, input: &mut ObjectDataInput) {
    self.publish_time = Some(input.read_long());
    self.publisher_address = Some(input.read_object().downcast().unwrap());
    self.payload = input.read_data();
  }

  fn write_data(&mut self, output: &mut ObjectDataOutput) {
    output.write_long(self.publish_time.unwrap());
    output.write_object(self.publisher_address.as_mut().unwrap().as_mut());
    output.write_data(self.payload.as_ref());
  }
}

pub fn reliable_topic_message_factory(class_id: i32) -> Box<dyn IdentifiedDataSerializable> {
  if class_id == RELIABLE_TOPIC_CLASS_ID {
    Box::new(ReliableTopicMessage::new())
  } else {
    todo!()
  }
}