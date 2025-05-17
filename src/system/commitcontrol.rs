use std::{collections::HashMap, path::PathBuf};
use serde::{Serialize, Deserialize};

use super::{change::Change, commit::Commit};

const UNTRACEABLE_COMMIT_NAME: &str = "UntraceableCommit";

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitControl {
  last_commit: String,
  commits: Box<HashMap<u16, String>>,
  #[serde(skip_serializing)]
  current_commit: Box<Commit>,
  counter: u16,
}

impl CommitControl {
  pub fn create(change: Change, parent: Option<String>) -> std::io::Result<CommitControl> {
    let mut map: HashMap<u16, String> = HashMap::new();
    map.insert(1, String::from(UNTRACEABLE_COMMIT_NAME));

    Ok(CommitControl { 
      last_commit: String::from(UNTRACEABLE_COMMIT_NAME), 
      commits: Box::new(map), 
      current_commit: Box::new(Commit::create(change, parent)?), 
      counter: 1,
    })
  }

  pub fn init(
    &mut self, name: String, message: String, path: PathBuf
  ) -> std::io::Result<()> {
    println!("{:?}", path);
    self.current_commit.init(name, message, path)?;
    Ok(())
  }
}