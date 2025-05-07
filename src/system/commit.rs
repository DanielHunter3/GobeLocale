use std::fs::File;
use::std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::io::{self};

use crate::system::change::Change;

#[derive(Debug, Serialize, Deserialize)]
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
  pub fn new(name: String, message: String, parent: Option<String>, change: Change) -> Commit { 
    Commit { 
      name, 
      message,
      parent,
      change: Box::new(change), 
      gci_file: PathBuf::new()
    } 
  }

  pub fn get_name(&self) -> String {
    self.name.clone()
  }

  pub fn get_message(&self) -> String {
    self.message.clone()
  }

  pub fn get_parent(&self) -> Option<String> {
    self.parent.clone()
  }

  pub fn save_to_file(&self, file: &PathBuf) -> io::Result<()> {
    let file = File::create(file)?;
    serde_json::to_writer(file, &self)?;
    Ok(())
  }

  pub fn load_from_file(path: &PathBuf) -> io::Result<Commit> {
    let file = File::open(path)?;
    let change = serde_json::from_reader(file)?;
    Ok(change)
  }

  // TODO
  pub fn save(&self) {}
  // TODO
  pub fn upload(&self) {}
}