use crate::system::versioncontrol::VersionControl;

pub struct Branch {
  name: String,
  last_commit: String,
  current_commit: String,
  commits: VersionControl,
}

impl Branch {
  pub fn new(name: String) -> Branch {
    Branch { 
      name, 
      last_commit: String::new(), 
      current_commit: String::new(), 
      commits: VersionControl::new() 
    }
  }
}