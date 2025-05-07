use crate::system::commit::Commit;

pub struct Branch {
  name: String,
  last_commit: String,
  current_commit: Box<Option<Commit>>,
  commits: Vec<String>,
}

impl Branch {
  pub fn new(name: &str) -> Branch {
    Branch { 
      name: String::from(name), 
      last_commit: String::new(), 
      current_commit: Box::new(None),
      commits: Vec::new()
    }
  }

  pub fn append(commit: Commit) {
    
  }
}
