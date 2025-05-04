use crate::system::branch::Branch;

use std::{fs, path::PathBuf};

pub struct LocaleRepository {
  // name of locale user
  author: String,
  // name of repository
  name: String,
  // Расположение проекта на этой машине
  project_sourse_dir: PathBuf,
  // Расположение папки .gobe для этого репозитория на этой машине
  repository_folder: PathBuf,
  // Версия репозитория (проекта)
  version: String,
  // Активная ветка
  current_branch: String,
  // Главная ветка
  main_branch: String,
  // Все ветки
  branches: Vec<String>,
  //----------------------
  
}

impl LocaleRepository {

  pub fn new(author: String, name: String) -> LocaleRepository {
    LocaleRepository { 
      author, 
      name, 
      project_sourse_dir: std::env::current_dir().unwrap(), 
      repository_folder: PathBuf::from(
        String::from(std::env::current_dir().unwrap().to_str().unwrap())+"/.gobe"
      ),
      version: "0.0.0".to_string(), 
      current_branch: String::from("master"), 
      main_branch: String::from("master"), 
      branches: vec![String::from("master")]
    }
  }

  pub fn init(&self) -> std::io::Result<()> {
    let file_branches = fs::File::create("")?;
    Ok(())
  }

}