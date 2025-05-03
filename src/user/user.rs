use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLocale {
  name: String,
  email: String,
  repos: Vec<String>
}

impl UserLocale {
  pub fn new(name: String, email: String) -> UserLocale {
    UserLocale { name, email, repos: Vec::new() }
  }
  pub fn register(name: String, email: String) -> UserLocale {
    Self::new(name, email)
    // call
  }
  pub fn login(name: String, email: String) -> UserLocale {
    Self::new(name, email)
    // call
  }
}