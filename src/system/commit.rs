use std::fs::File;
use::std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::io::{self};

use super::change::Change;
use super::folder::Folder;

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
  folder: Box<Folder>
}

impl Commit {
  pub fn create(change: Change, parent: Option<String>) -> std::io::Result<Commit>
  {
    Ok(Commit { 
      name: String::new(), 
      message: String::new(),
      parent,
      change: Box::new(change), 
      folder: Box::new(Folder::new(None)?)
    })
  }

  fn load(&self, is_upload: bool) -> std::io::Result<()> {
    self.save_info_file()?;
    if is_upload { self.upload()? }
    Ok(())
  }

  pub fn init(
    &mut self, name: String, message: String, path: PathBuf
  ) -> std::io::Result<()> {
    self.name = name.clone();
    self.message = message;
    self.folder.init(path.join(name))?;

    println!("{:?}", self.folder);

    self.load(false)?;

    Ok(())
  }

  pub fn save_info_file(&self) -> io::Result<()> {
    let file = File::create(
      &self.folder.object_folder()?.join(self.name.clone() + ".json")
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