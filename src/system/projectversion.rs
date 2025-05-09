use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectVersion {
  senior: usize,
  middle: usize,
  junior: usize,
  devstage: usize
}

impl ProjectVersion {
  pub fn new(senior: usize, middle: usize, junior: usize, devstage: usize) -> ProjectVersion {
    ProjectVersion { senior, middle, junior, devstage }
  }

  pub fn to_string(&self) -> String {
    format!("{}.{}.{}.{}", self.senior, self.middle, self.junior, self.devstage)
  }
}