use::std::path::PathBuf;

use crate::system::change::Change;

pub struct Commit {
  name: String,
  //
  message: String,
  // time pub
  pub change: Change,
  // time pub
  pub gci_file: PathBuf
}

impl Commit {
  pub fn new(name: String, message: String, change: Change) -> Commit { 
    Commit { 
      name: name.to_string(), 
      message, 
      change, 
      gci_file: PathBuf::from(name + ".json")
    } 
  }

  // TODO
  pub fn save() {}
  // TODO
  pub fn upload() {}
}