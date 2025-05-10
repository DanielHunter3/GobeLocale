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
  version: Box<ProjectVersion>,
  // Активная ветка (может и не быть никакой активной ветки)
  #[serde(skip_serializing)]
  current_branch: Box<Option<Branch>>,
  // Главная ветка
  main_branch: String,
  // Все ветки
  branches: Vec<String>,
}

impl LocaleRepository {

  fn alloc(author: String, name: String) -> std::io::Result<LocaleRepository> {
    // Check if current directory exists and is accessible
    let current_dir = std::env::current_dir()?;
    
    // Create the repository path
    let repo_folder = current_dir.join(".gobe");
    //
    Ok(LocaleRepository {
      // Должен быть локальный юзер
      author, 
      // Название репозитория
      name,
      //---------------------------------------------------------------------
      // Положение на этой машине
      project_sourse_dir: current_dir, 
      // См. выше
      repository_folder: repo_folder,
      //---------------------------------------------------------------------
      // Нулевая готовность проекта
      version: Box::new(ProjectVersion::new(0, 0, 0, 0)),
      //----------------------------------------------------------------------
      // Текущая ветка: None
      current_branch: Box::new(None),
      // Главная ветка: NULL
      main_branch: String::new(), 
      // Новые ветки: NULL
      branches: Vec::new()
    })
  }

  fn init(&mut self) -> std::io::Result<()> {
    self.current_branch = Box::new(
      Some(
        Branch::new(
        String::from("master"), 
        self.repository_folder.clone()
        )?
      )
    );
    self.main_branch = String::from("master");
    self.branches.push(String::from("master"));
    Ok(())
  }

  pub fn new(author: String, name: String) -> std::io::Result<LocaleRepository> {
    //
    std::fs::create_dir_all(std::env::current_dir()?.join(".gobe"))?;
    //
    let mut result = Self::alloc(author, name)?;
    result.init()?;
    //
    result.save_info_file()?;
    //
    Ok(result)
  }

  #[allow(dead_code)]
  pub fn append(&mut self, name: String) {
    
  }

  pub fn save_info_file(&self) -> std::io::Result<()> {
    let file_path = self.repository_folder.join(self.name.clone() + ".json");
    let file = File::create(&file_path)?;
    serde_json::to_writer(file, &self)?;
    Ok(())
  }

  pub fn load_from_file(path: &PathBuf) -> std::io::Result<LocaleRepository> {
    let file = File::open(path)?;
    let change = serde_json::from_reader(file)?;
    Ok(change)
  }

  // for tests
  pub fn get_repo_folder(&self) -> PathBuf {
    self.repository_folder.clone()
  }

  pub fn upload(&self) -> std::io::Result<()> {
    Ok(())
  }

}