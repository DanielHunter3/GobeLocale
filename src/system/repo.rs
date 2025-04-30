use crate::system::branch::Branch;

use std::{fs, path::PathBuf};

pub struct LocaleRepository {
  //-------------------
  author: String,
  name: String,
  //-------------------
  project_sourse_dir: PathBuf,
  //------
  version: String,
  //------
  current_branch: String, // names of branches
  main_branch: String,
  branches: Vec<String>,
  //----------------------
  
}

impl LocaleRepository {

  pub fn new(author: String, name: String) -> LocaleRepository {
    LocaleRepository { 
      author, 
      name, 
      project_sourse_dir: std::env::current_dir().unwrap(), 
      version: "0.0.0".to_string(), 
      current_branch: String::new(), 
      main_branch: String::new(), 
      branches: Vec::new() 
    }
  }

  pub fn init() -> Option<bool> {
    if fs::exists("branches.gci").unwrap() { return None; }
    let file_branches = fs::File::create("branches.gci");
    if file_branches.is_err() { return Some(false); }
    Some(true)
  }

}