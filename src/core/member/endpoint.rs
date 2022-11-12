#[derive(Clone, Eq, PartialEq, Hash)]
pub struct EndpointQualifier {
  pub _type: i32,
  pub identifier: Option<String>,
}

impl EndpointQualifier {
  pub fn new(_type: i32, identifier: Option<String>) -> Self {
    EndpointQualifier {
      _type,
      identifier,
    }
  }

  pub fn equals(&self, other: EndpointQualifier) -> bool {
    self._type == other._type && self.identifier == other.identifier
  }
}
