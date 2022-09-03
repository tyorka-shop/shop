
pub struct ID(pub String);

impl From<&str> for ID {
  fn from(s: &str) -> Self {
      Self(s.to_string())
  }
}

impl From<ID> for String {
  fn from(id: ID) -> Self {
      id.0
  }
}