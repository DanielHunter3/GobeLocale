use crate::system::branch::Branch;

use std::fs;

pub struct LocaleRepository {
  //-------------------
  author: String,
  name: String,
  //-------------------
  path: String,
  project_path: String,
  //------
  version: String,
  //------
  current_branch: Branch,
  main_branch: Branch,
  branches: Vec<Branch>,
  //----------------------
}

impl LocaleRepository {

  pub fn init() -> Option<bool> {
    if fs::exists("branches.gci").unwrap() { return None; }
    let file_branches = fs::File::create("branches.gci");
    if file_branches.is_err() { return Some(false); }
    Some(true)
  }

}