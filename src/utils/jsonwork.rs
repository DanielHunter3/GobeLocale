use std::path::is_separator;

use serde_json::{json, Value};

struct JsonFile {
  json_data: Value,
}

impl JsonFile {
  pub fn insert_value(&mut self, key: &str, value: &str) -> bool
  {
    if let Some(obj) = self.json_data.as_object_mut() {
      obj.insert(key.to_string(), json!(value));
      return true;
    }
    false
  }

  pub fn remove_value(&mut self, key: &str) -> Option<bool> {
    if let Some(obj) = self.json_data.as_object_mut() {
      let mut is_find = false;
      for i in obj.keys() {
        if &key.to_string() == i { is_find = true; }
      }
      if !is_find { return None; }
      obj.remove(key);
      return Some(true);
    }
    Some(false)
  }
}