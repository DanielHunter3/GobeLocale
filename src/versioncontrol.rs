use std::collections::HashMap;

use crate::commit::Commit;

pub struct VersionControl {
  commits: HashMap<Commit, u32>,
}

impl VersionControl {
  pub fn new() -> VersionControl {
    VersionControl { commits: HashMap::new() }
  }
}