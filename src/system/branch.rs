use std::{fs::File, path::PathBuf};
use serde::{Deserialize, Serialize};

use super::change::Change;
use super::commitcontrol::CommitControl;
use super::folder::Folder;

#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
  name: String,
  commit_control: Box<CommitControl>, 
  folder: Box<Folder>
}

impl Branch {
  pub fn create(name: String, path: PathBuf) -> std::io::Result<Branch> {
    let first_commit_name = name.clone() + "_first";

    let mut result = Ok(Branch { 
      name: name.clone(),
      commit_control: Box::new(CommitControl::create(Self::create_change(), None)?),
      folder: Box::new(Folder::new(Some(path.join(name)))?)
    });

    //--------------------------------

    // TODO
    if let Ok(branch) = &mut result {
      branch.commit_control.init(
        first_commit_name.clone(), 
        String::from("new commit"), 
        branch.folder.object_folder.clone().unwrap()
      )?;
      branch.load(false)?
    }

    //-------------------------------

    result
  }

  fn load(&self, is_upload: bool) -> std::io::Result<()> {
    self.save_info_file()?;
    if is_upload { self.upload()? }
    Ok(())
  }

    // TODO
  pub fn upload(&self) -> std::io::Result<()> {
    Ok(())
  }

  pub fn save_info_file(&self) -> std::io::Result<()> {
    let file = File::create(
      &self.folder.object_folder()?.join(self.name.clone() + ".json")
    )?;
    serde_json::to_writer(file, &self)?;
    Ok(())
  }

  //TODO
  pub fn append(&mut self, name_commit: String, change: Change) -> std::io::Result<()> {
    //if commit.get_parent().is_none() && !self.commits.is_empty() { return Err("to-to-to"); }
    //commit.save_to_file();
    //self.commits.insert(self.commits.len()+1, commit.get_name());
    Ok(())
  }

  fn create_change() -> Change {
    Change::new()
  }
}