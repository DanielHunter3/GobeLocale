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
  change: Box<Change>,
  // commit folder
  commit_folder: PathBuf
}

impl Commit {
  fn alloc(
    name: String, message: String, change: Change, parent: Option<String>, path: PathBuf
  ) -> std::io::Result<Commit>
  {
    std::fs::create_dir(path.join(&name))?;
    Ok(Commit { 
      name: name.clone(), 
      message,
      parent,
      change: Box::new(change), 
      commit_folder: path.join(name)
    })
  }

  fn init(&self, is_upload: bool) -> std::io::Result<()> {
    self.save_info_file()?;
    if is_upload { self.upload()? }
    Ok(())
  }

  pub fn new(
    name: String, message: String, change: Change, parent: Option<String>, path: PathBuf
  ) -> std::io::Result<Commit> {
    let /* mut */ result = Self::alloc(name, message, change, parent, path);
    if let Ok(ref /* mut */ commit) = result { 
      commit.init(false)?;
    }
    result
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