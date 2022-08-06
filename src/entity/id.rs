
pub struct ID(pub String);

impl From<&str> for ID {
  fn from(s: &str) -> Self {
      Self(s.to_string())
  }
}