use std::fs::File;
use std::io::{self};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

use crate::utils::utilvec::remove_from_index;

pub enum ChangeVariant {
  UNTRACEABLE,
  DELETE,
  MODIFIED
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Change {
  untraceable_files: Vec<String>,
  delete_files: Vec<String>,
  modified_files: Vec<String>,
}

impl Change {
  pub fn new() -> Change {
    Change {
      untraceable_files: Vec::new(),
      delete_files: Vec::new(),
      modified_files: Vec::new(),
    }
  }

  pub fn append(&mut self, file: String, variant: ChangeVariant) {
    match variant {
      ChangeVariant::UNTRACEABLE => self.untraceable_files.push(file),
      ChangeVariant::DELETE => self.delete_files.push(file),
      ChangeVariant::MODIFIED => self.modified_files.push(file)
    }
  }

  pub fn remove(&mut self, file: String, variant: ChangeVariant) {
    match variant {
      ChangeVariant::UNTRACEABLE => remove_from_index(&mut self.untraceable_files, file),
      ChangeVariant::DELETE => remove_from_index(&mut self.delete_files, file),
      ChangeVariant::MODIFIED => remove_from_index(&mut self.modified_files, file),
    }
  }

  // Метод для сохранения изменений в файл в формате JSON
  pub fn save_to_file(&self, path: &PathBuf) -> io::Result<()> {
    let file = File::create(path)?;
    serde_json::to_writer(file, self)?;
    Ok(())
  }

  // Метод для загрузки изменений из файла в формате JSON
  pub fn load_from_file(path: &PathBuf) -> io::Result<Change> {
    let file = File::open(path)?;
    let change = serde_json::from_reader(file)?;
    Ok(change)
  }
}
