use std::{collections::HashMap, fs::File, path::PathBuf};
use serde::{Deserialize, Serialize};

use crate::system::commit::Commit;
use crate::system::change::Change;

#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
  name: String,
  last_commit: String,
  #[serde(skip_serializing)]
  current_commit: Box<Option<Commit>>,
  commits: Box<HashMap<usize, String>>,
  branch_folder: PathBuf
}

impl Branch {
  pub fn alloc(name: String, path: PathBuf) -> Branch {
    Branch { 
      name: name.clone(), // init
      last_commit: String::new(),
      current_commit: Box::new(None),
      commits: Box::new(HashMap::new()), // init
      branch_folder: path.join(name) // init
    }
  }

  fn init(&mut self) -> std::io::Result<()> {
    let change = Change::new(); // TODO
    let name = self.name.clone() + "_first";

    self.current_commit = Box::new(
      Some(
        Commit::new(
          name.clone(), None, change, None, self.branch_folder.clone()
        )?
      )
    );
    self.last_commit = String::from(&name);
    self.commits.insert(1, name);

    Ok(())
  }

  pub fn new(name: String, path: PathBuf) -> std::io::Result<Branch> {
    //
    std::fs::create_dir(path.join(&name))?;
    //
    let mut result = Self::alloc(name, path);
    result.init()?;
    //
    result.save_info_file()?;
    //
    Ok(result)
  }

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
}