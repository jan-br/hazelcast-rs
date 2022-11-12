use std::collections::HashMap;
use crate::serialization::compact::rabin_fingerprint64::RabinFingerprint64;
use crate::serialization::generic_record::field_descriptor::FieldDescriptor;
use crate::serialization::generic_record::field_kind::FieldKind;
use crate::serialization::generic_record::field_operations::FieldOperations;
use crate::util::bits_util::BitsUtil;

#[derive(Clone)]
pub struct Schema {
  pub type_name: String,
  pub field_definition_map: HashMap<String, FieldDescriptor>,
  pub fields: Vec<FieldDescriptor>,
  pub number_var_size_fields: usize,
  pub fixed_size_fields_length: usize,
  pub schema_id: i64,
}

impl Schema {
  pub fn new(type_name: String, fields: Vec<FieldDescriptor>) -> Self {
    let mut field_definition_map = HashMap::new();
    for field in &fields {
      field_definition_map.insert(field.field_name.clone(), field.clone());
    }

    Schema {
      type_name,
      field_definition_map,
      fields,
      number_var_size_fields: 0,
      fixed_size_fields_length: 0,
      schema_id: 0,
    }
  }

  pub fn init(&mut self) {
    let mut fixed_size_fields = vec![];
    let mut boolean_fields = vec![];
    let mut variable_size_fields = vec![];

    for (_, descriptor) in &self.field_definition_map {
      let field_kind = &(unsafe { std::mem::transmute::<_, FieldKind>(descriptor.kind) });
      if field_kind.get_operation().kind_size_in_bytes() == FieldOperations::VARIABLE_SIZE as usize {
        variable_size_fields.push(descriptor.clone());
      } else if field_kind == &FieldKind::Boolean {
        boolean_fields.push(descriptor.clone());
      } else {
        fixed_size_fields.push(descriptor.clone());
      }
    }

    fixed_size_fields.sort_by(
      |a, b| unsafe { std::mem::transmute::<_, FieldKind>(b.kind) }.get_operation().kind_size_in_bytes()
          .cmp(&unsafe { std::mem::transmute::<_, FieldKind>(a.kind) }.get_operation().kind_size_in_bytes()));

    let mut offset = 0;
    for mut descriptor in fixed_size_fields {
      descriptor.offset = offset;
      offset += (unsafe { std::mem::transmute::<_, FieldKind>(descriptor.kind) }).get_operation().kind_size_in_bytes() as isize;
    }

    let mut bit_offset = 0;
    for mut descriptor in boolean_fields {
      descriptor.offset = offset;
      descriptor.bit_offset = (bit_offset % BitsUtil::BITS_IN_A_BYTE) as isize;
      bit_offset += 1;
      if bit_offset % BitsUtil::BITS_IN_A_BYTE != 0 {
        offset += 1;
      }
    }
    if bit_offset % BitsUtil::BITS_IN_A_BYTE != 0 {
      offset += 1;
    }

    self.fixed_size_fields_length = offset as usize;
    let mut index = 0;
    for mut descriptor in variable_size_fields {
      descriptor.index = index;
      index += 1;
    }

    self.number_var_size_fields = index as usize;
    self.schema_id = RabinFingerprint64::of_schema(&self);
  }

  pub fn get_fields(&self) -> Vec<FieldDescriptor> {
    self.fields.clone()
  }

  fn has_same_fields(&self, other: &Schema) -> bool {
    if other.field_definition_map.len() != self.field_definition_map.len() {
      return false;
    }
    for (field_name, field) in &self.field_definition_map {
      if !other.field_definition_map.contains_key(field_name) {
        return false;
      }
      let other_field = other.field_definition_map.get(field_name).unwrap();
      if other_field.kind != field.kind {
        return false;
      }
    }
    true
  }

  pub fn equals(&self, other: &Schema) -> bool {
    if self.schema_id != other.schema_id {
      return false;
    }
    if self.type_name != other.type_name {
      return false;
    }
    if self.fixed_size_fields_length != other.fixed_size_fields_length {
      return false;
    }
    if self.number_var_size_fields != other.number_var_size_fields {
      return false;
    }
    self.has_same_fields(other)
  }
}