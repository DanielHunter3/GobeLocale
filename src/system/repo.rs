use crate::system::branch::Branch;
use crate::system::projectversion::ProjectVersion;

use serde::{Deserialize, Serialize};
use std::{fs::File, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
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
  version: ProjectVersion,
  // Активная ветка
  #[serde(skip_serializing)]
  current_branch: Box<Branch>,
  // Главная ветка
  main_branch: String,
  // Все ветки
  branches: Vec<String>,
}

impl LocaleRepository {

  pub fn new(author: String, name: String) -> LocaleRepository {
    LocaleRepository {
      // Должен быть локальный юзер
      author, 
      // Название репозитория
      name,
      // Положение на этой машине
      project_sourse_dir: std::env::current_dir().unwrap(), 
      // См. выше
      repository_folder: 
        std::env::current_dir().unwrap().join(".gobe"),
      // Нулевая готовность проекта
      version: ProjectVersion::new(0, 0, 0, 0),
      // Текущая ветка: master
      current_branch: Box::new(Branch::new(String::from("master"))), 
      // Главная ветка: master
      main_branch: String::from("master"), 
      // Новые ветки: добаляем master
      branches: vec![String::from("master")]
    }
  }

  pub fn init(&self) -> std::io::Result<()> {
    // TODO
    std::fs::create_dir(&self.repository_folder)?;
    self.save_to_file()?;
    Ok(())
  }

  pub fn save_to_file(&self) -> std::io::Result<()> {
    let file = File::create(
      self.repository_folder.clone().join(self.name.clone() + ".json")
    )?;
    serde_json::to_writer(file, &self)?;
    Ok(())
  }

  pub fn load_from_file(path: &PathBuf) -> std::io::Result<LocaleRepository> {
    let file = File::open(path)?;
    let change = serde_json::from_reader(file)?;
    Ok(change)
  }

  // for tests
  pub fn get_repo_folder(&self) -> &str {
    self.repository_folder.to_str().unwrap()
  }

}