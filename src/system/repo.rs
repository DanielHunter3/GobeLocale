use crate::system::branch::Branch;
use crate::system::projectversion::ProjectVersion;

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
  version: ProjectVersion,
  // Активная ветка
  current_branch: Branch,
  // Главная ветка
  main_branch: String,
  // Все ветки
  branches: Vec<String>,
  //----------------------
  
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
      repository_folder: PathBuf::from(
        String::from(std::env::current_dir().unwrap().to_str().unwrap())+"/.gobe"
      ),
      // Нулевая готовность проекта
      version: ProjectVersion::new(0, 0, 0, 0),
      // Текущая ветка: master
      current_branch: Branch::new(String::from("master")), 
      // Главная ветка: master
      main_branch: String::from("master"), 
      // Новые ветки: добаляем master
      branches: vec![String::from("master")]
    }
  }

  pub fn init(&self) -> std::io::Result<()> {
    // TODO
    let file_branches = fs::File::create("")?;
    Ok(())
  }

}