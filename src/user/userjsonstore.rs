use crate::user::user::UserLocale;

use serde::de::Error;
use serde_json::Result;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

pub trait JsonStoreTrait {
  fn create(file_path: &str) -> Result<Self> where Self: Sized;
  fn load(file_path: &str) -> Result<Self> where Self: Sized;
  fn save(&self, file_path: &str) -> Result<()>;
  fn add_person(&mut self, key: String, new_person: UserLocale);
  fn remove_person(&mut self, key: &str) -> Result<()>;
  fn update_person(&mut self, key: &str, updated_person: UserLocale) -> Result<()>;
}

// Убираем обертку JsonStore
pub type JsonStore = HashMap<String, UserLocale>;

impl JsonStoreTrait for JsonStore {
  fn create(file_path: &str) -> Result<Self> {
    let store = JsonStore::new();
    let json_data = serde_json::to_string(&store).unwrap();
    let mut file = File::create(file_path).unwrap();
    file.write_all(json_data.as_bytes()).unwrap();
    Ok(store)
  }

  fn load(file_path: &str) -> Result<Self> {
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let store: JsonStore = serde_json::from_str(&contents).unwrap();
    Ok(store)
  }

  fn save(&self, file_path: &str) -> Result<()> {
    let json_data = serde_json::to_string_pretty(self).unwrap(); // Используем pretty для удобного формата
    let mut file = File::create(file_path).unwrap();
    file.write_all(json_data.as_bytes()).unwrap();
    Ok(())
  }

  fn add_person(&mut self, key: String, new_person: UserLocale) {
    self.insert(key, new_person);
  }

  fn remove_person(&mut self, key: &str) -> Result<()> {
    if self.remove(key).is_none() {
      Err(serde_json::Error::custom("Key not found"))
    } else {
      Ok(())
    }
  }

  fn update_person(&mut self, key: &str, updated_person: UserLocale) -> Result<()> {
    if self.contains_key(key) {
      self.insert(key.to_string(), updated_person);
      Ok(())
    } else {
      Err(serde_json::Error::custom("Key not found"))
    }
  }
}