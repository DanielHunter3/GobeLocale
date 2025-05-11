use std::{collections::HashMap, fs::File, path::PathBuf};
use serde::{Deserialize, Serialize};

use crate::system::commit::Commit;
use crate::system::change::Change;

#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
  name: String,
  last_commit: String,
  #[serde(skip_serializing)]
  current_commit: Box<Commit>,
  commits: Box<HashMap<usize, String>>,
  branch_folder: PathBuf
}

impl Branch {
  pub fn alloc(name: String, path: PathBuf) -> std::io::Result<Branch> {
    std::fs::create_dir(path.join(&name))?;

    let xname = name.clone() + "_first";
    let mut commits = HashMap::new();
    commits.insert(1, xname.clone());

    Ok(Branch { 
      name: name.clone(),
      last_commit: xname.clone(),

      current_commit: 
      Box::new(
        Commit::new(
          xname.clone(), String::from("first commit"), 
          Self::alloc_change(), None, path.join(&name)
        )?
      ),

      commits: Box::new(commits),
      branch_folder: path.join(name)
    })
  }

  fn init(&self, is_upload: bool) -> std::io::Result<()> {
    self.save_info_file()?;
    if is_upload { self.upload()? }
    Ok(())
  }

  pub fn new(name: String, path: PathBuf) -> std::io::Result<Branch> {
    let /* mut */ result = Self::alloc(name, path);
    if let Ok(ref /* mut */ branch) = result { // mut
      branch.init(false)?;
    }
    result
  }

    // TODO
  pub fn upload(&self) -> std::io::Result<()> {
    Ok(())
  }

  pub fn save_info_file(&self) -> std::io::Result<()> {
    let file = File::create(
      &self.branch_folder.join(self.name.clone() + ".json")
    )?;
    serde_json::to_writer(file, &self)?;
    Ok(())
  }

  pub fn get_name(&self) -> String {
    self.name.clone()
  }

  //TODO
  pub fn append(&mut self, name_commit: String, change: Change) -> std::io::Result<()> {
    //if commit.get_parent().is_none() && !self.commits.is_empty() { return Err("to-to-to"); }
    //commit.save_to_file();
    //self.commits.insert(self.commits.len()+1, commit.get_name());
    Ok(())
  }

  fn alloc_change() -> Change {
    Change::new()
  }
}