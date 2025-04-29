use::std::path::PathBuf;

use crate::change::Change;

pub struct Commit {
  name: String,
  //
  message: String,
  //
  change: Change,
  //
  gci_file: PathBuf
}

impl Commit {
  pub fn new(name: String, 
            message: String, 
            change: Change) -> Commit 
  { 
    Commit { 
      name: name.to_string(), 
      message, 
      change, 
      gci_file: PathBuf::from(name + ".gci")
    } 
  }
}