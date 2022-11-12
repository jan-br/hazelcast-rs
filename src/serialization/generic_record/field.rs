use crate::serialization::generic_record::field_kind::FieldKind;

pub trait Field {
  fn kind(&self) -> FieldKind;
}

pub type FieldWithValue<T> = (Box<dyn Field>, T);