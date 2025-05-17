use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Title {
  author: String,
  assistants: Vec<String>,
  name: String
}

impl Title {
  pub fn create(name: String) -> std::io::Result<Title> {
    Ok(Title {
      // В будущем будет функция, возвращающая std::io::Result<String>
      author: String::new(),
      assistants: Vec::new(),
      name
    })
  }

  pub fn name(&self) -> String {
    self.name.clone()
  }
}