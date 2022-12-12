use std::sync::Arc;
use crate::core::member::Member;

pub struct EntryEvent<K, V> {
  pub name: String,
  pub key: Option<K>,
  pub value: Option<V>,
  pub old_value: Option<V>,
  pub merging_value: Option<V>,
  pub member: Option<Arc<Member>>,
}

impl<K, V> EntryEvent<K, V> {
  pub fn new(name: String, key: Option<K>, value: Option<V>, old_value: Option<V>, merging_value: Option<V>, member: Option<Arc<Member>>) -> Self {
    Self {
      name,
      key,
      value,
      old_value,
      merging_value,
      member,
    }
  }
}