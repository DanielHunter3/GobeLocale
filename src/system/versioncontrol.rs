use crate::system::commit::Commit;

use std::collections::HashMap;

pub struct VersionControl {
  commits: HashMap<u32, Commit>,
}

impl VersionControl {
  pub fn new() -> VersionControl {
    VersionControl { commits: HashMap::new() }
  }
}