use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectVersion {
  senior: u8,
  middle: u8,
  junior: u8,
  devstage: u8
}

impl ProjectVersion {
  pub fn create(senior: u8, middle: u8, junior: u8, devstage: u8) -> ProjectVersion {
    ProjectVersion { senior, middle, junior, devstage }
  }
}