use std::{io::ErrorKind, path::PathBuf};
use std::fs::create_dir_all;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Folder {
  pub object_folder: Option<PathBuf>,
  pub project_folder: PathBuf
}

impl Folder {
  pub fn new(some_path: Option<PathBuf>) -> std::io::Result<Folder> {
    if let Some(path) = &some_path {
      create_dir_all(path)?
    }
    Ok(Folder {
      object_folder: some_path,
      project_folder: std::env::current_dir()?
    })
  }

  pub fn object_folder(&self) -> std::io::Result<&PathBuf> {
    if let Some(ret) = &self.object_folder {
      Ok(ret)
    } else {
      Err(ErrorKind::NotSeekable.into())
    }
  }

  pub fn init(&mut self, o_folder: PathBuf) -> std::io::Result<()> {
    create_dir_all(&o_folder)?;
    self.object_folder = Some(o_folder);
    Ok(())
  }
}