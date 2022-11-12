use crate::config::serialization::SerializationConfig;
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::portable::PortableSerializable;
use crate::serialization::portable::portable_context::PortableContext;
use crate::serialization::serializer::Serializer;

pub struct PortableSerializer {
  pub portable_context: PortableContext,
}

impl PortableSerializer {
  pub fn new(config: SerializationConfig) -> Self {
    Self {
      portable_context: PortableContext::new(config.portable_version)
    }
  }
}

impl Serializer<Box<dyn PortableSerializable>> for PortableSerializer {
  fn id(&self) -> i32 {
    -1
  }

  fn read(&self, input: &mut ObjectDataInput) -> Box<dyn PortableSerializable> {
    todo!()
  }

  fn write(&self, output: &mut ObjectDataOutput, object: Box<dyn PortableSerializable>) {
    todo!()
  }
}