use std::collections::HashMap;
use crate::serialization::portable::class_definition_context::ClassDefinitionContext;

pub struct PortableContext {
  pub version: i32,
  pub class_def_context: HashMap<i32, ClassDefinitionContext>,
}

impl PortableContext {
  pub fn new(version: i32) -> Self {
    Self {
      version,
      class_def_context: HashMap::new(),
    }
  }
}