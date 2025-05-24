use std::path::PathBuf;

use serde::{Serialize, Deserialize};

use super::branch::Branch;

#[derive(Debug, Serialize, Deserialize)]
pub struct BranchControl {
  #[serde(skip_serializing)]
  current_branch: Box<Branch>,
  main_branch: String,
  branches: Vec<String>,
}

impl BranchControl {
  pub fn create(path: PathBuf) -> std::io::Result<BranchControl> {
    Ok(
      BranchControl { 
        current_branch: Box::new(
          Branch::create(String::from("master"), path)?
        ), 
        main_branch: String::from("master"), 
        branches: vec![String::from("master")], 
      }
    )
  }
}