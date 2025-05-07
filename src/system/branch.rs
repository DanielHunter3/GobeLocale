use std::collections::HashMap;

use crate::system::commit::Commit;

pub struct Branch {
  name: String,
  last_commit: String,
  current_commit: Box<Option<Commit>>,
  commits: HashMap<usize, String>,
}

impl Branch {
  pub fn new(name: String) -> Branch {
    Branch { 
      name, 
      last_commit: String::new(), 
      current_commit: Box::new(None),
      commits: HashMap::new()
    }
  }
  //TODO
  pub fn append(&mut self, commit: &Commit) -> Result<(), &str> {
    if commit.get_parent().is_none() && !self.commits.is_empty() { return Err("to-to-to"); }
    commit.save();
    self.commits.insert(self.commits.len()+1, commit.get_name());
    Ok(())
  }
}
