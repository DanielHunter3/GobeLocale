// use std::path::PathBuf;

// use crate::utils::jsonwork;

pub struct UserLocale {
  name: String,
  email: String,
  repos: Vec<String>
}

impl UserLocale {
  pub fn new(name: String, email: String) -> UserLocale {
    //let mut json = jsonwork::JsonFile::new(PathBuf::from("."));
    //json.insert_value("repos", "[]");
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