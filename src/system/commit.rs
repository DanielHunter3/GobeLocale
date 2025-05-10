use std::fs::File;
use::std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::io::{self};

use crate::system::change::Change;

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
  name: String,
  //
  message: Option<String>,
  // parent commit
  parent: Option<String>,
  // time pub
  change: Box<Change>,
  // commit folder
  commit_folder: PathBuf
}

impl Commit {
  fn alloc(name: String, message: Option<String>, change: Change) -> Commit
  {
    Commit { 
      name, 
      message,
      parent: None,
      change: Box::new(change), 
      commit_folder: PathBuf::new()
    }
  }

  fn init(&mut self, parent: Option<String>, path: PathBuf) {
    self.parent = parent;
    self.commit_folder = path.join(&self.name);
  }

  pub fn new(
    name: String, message: Option<String>, change: Change, parent: Option<String>, path: PathBuf
  ) -> std::io::Result<Commit> {
    //
    std::fs::create_dir(path.join(&name))?;
    //
    let mut result = Self::alloc(name, message, change);
    result.init(parent, path);
    //
    result.save_info_file()?;
    //
    Ok(result)
  }

  pub fn get_name(&self) -> String {
    self.name.clone()
  }

  pub fn get_message(&self) -> String {
    self.message.clone().unwrap().clone()
  }

  pub fn get_parent(&self) -> Option<String> {
    self.parent.clone()
  }

  pub fn save_info_file(&self) -> io::Result<()> {
    let file = File::create(
      &self.commit_folder.join(self.name.clone() + ".json")
    )?;
    serde_json::to_writer(file, &self)?;
    Ok(())
  }

  pub fn load_from_file(path: &PathBuf) -> io::Result<Commit> {
    let file = File::open(path)?;
    let change = serde_json::from_reader(file)?;
    Ok(change)
  }

  // TODO
  pub fn upload(&self) -> std::io::Result<()> {
    Ok(())
  }
}