use crate::serialization::generic_record::field_kind::FieldKind;

#[derive(Clone)]
pub struct FieldDescriptor {
  pub index: isize,
  pub offset: isize,
  pub bit_offset: isize,
  pub field_name: String,
  pub kind: i32,
}

impl FieldDescriptor {
  pub fn new(field_name: String, kind: impl Into<FieldKind>) -> Self {
    FieldDescriptor {
      index: -1,
      offset: -1,
      bit_offset: -1,
      field_name,
      kind: kind.into() as i32,
    }
  }

  pub fn equals(&self, other: &FieldDescriptor) -> bool {
    self.field_name == other.field_name && self.kind == other.kind
  }
}