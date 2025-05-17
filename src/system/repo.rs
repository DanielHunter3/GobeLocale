use super::projectversion::ProjectVersion;
use super::folder::Folder;
use super::branchcontrol::BranchControl;
use super::title::Title;

use serde::{Deserialize, Serialize};
use std::{fs::File, env::current_dir, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct LocaleRepository {
  // name of repository
  title: Box<Title>,
  // Расположение проекта на этой машине
  // Расположение папки .gobe для этого репозитория на этой машине
  folder: Box<Folder>,
  // Версия репозитория (проекта)
  version: Box<ProjectVersion>,
  // Активная ветка (может и не быть никакой активной ветки)
  branch_control: Box<BranchControl>,
}

impl LocaleRepository {

  pub fn create(name: String) -> std::io::Result<LocaleRepository> {
    let repo_folder = current_dir()?.join(".gobe");
    let result = Ok(LocaleRepository {
      // * Название репозитория
      title: Box::new(Title::create(name)?),
      // * Положение на этой машине
      folder: Box::new(Folder::new(Some(repo_folder.clone()))?),
      // * Нулевая готовность проекта
      version: Box::new(ProjectVersion::create(0, 0, 0, 0)),
      // * Контроль над ветками
      branch_control: Box::new(BranchControl::create(repo_folder)?)
    });

    if let Ok(ref /* mut */ repo) = result {
      repo.load(false)?
    }

    result
  }

  fn load(&self, is_upload: bool) -> std::io::Result<()> {
    self.save_info_file()?;
    if is_upload { self.upload()? }
    Ok(())
  }

  pub fn append(&mut self, name: String) {
    // TODO
  }

  pub fn save_info_file(&self) -> std::io::Result<()> {
    let file_path = 
      self.folder.object_folder()?
        .join(self.title.name().clone() + ".json");
    let file = File::create(&file_path)?;
    serde_json::to_writer(file, &self)?;
    Ok(())
  }

  pub fn load_from_file(path: &PathBuf) -> std::io::Result<LocaleRepository> {
    let file = File::open(path)?;
    let change = serde_json::from_reader(file)?;
    Ok(change)
  }

  pub fn upload(&self) -> std::io::Result<()> {
    Ok(())
  }

}