use::std::path::PathBuf;

use crate::system::change::Change;

pub struct Commit {
  name: String,
  //
  message: String,
  // parent commit
  parent: Option<String>,
  // time pub
  pub change: Box<Change>,
  // time pub
  pub gci_file: PathBuf
}

impl Commit {
  pub fn new(name: &str, message: &str, parent: Option<String>, change: Change) -> Commit { 
    Commit { 
      name: String::from(name), 
      message: String::from(message), 
      parent,
      change: Box::new(change), 
      gci_file: PathBuf::from(name.to_string() + ".json")
    } 
  }

  pub fn get_name(&self) -> String {
    self.name.clone()
  }

  // TODO
  pub fn save() {}
  // TODO
  pub fn upload() {}
}