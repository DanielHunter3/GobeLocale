use std::fs::File;
use std::io::{self, Write};
use zip::{ZipWriter, write::FileOptions, ZipArchive};
use std::path::Path;

/*

fn create_zip(zip_path: &str, file_path: &str) -> io::Result<()> {
  let zip_file = File::create(zip_path)?;
  let mut zip_writer = ZipWriter::new(zip_file);

    // Опции для файла в ZIP
  let options = FileOptions::default()
      .compression_method(zip::CompressionMethod::Stored)
      .unix_permissions(0o755)
      .large_file(false);

  // Добавляем файл в архив
  zip_writer.start_file(Path::new(file_path).file_name().unwrap().to_string_lossy(), options)?;
    
  // Читаем содержимое файла и записываем его в архив
  let mut file = File::open(file_path)?;
  io::copy(&mut file, &mut zip_writer)?;

  // Закрываем архив
  zip_writer.finish()?;
  Ok(())
}

*/

fn create_zip(zip_path: &str, files: Vec<&str>) -> io::Result<()> {
  // Создаем ZIP-файл
  let zip_file = File::create(zip_path)?;
  let mut zip_writer = ZipWriter::new(zip_file);

  // Настройки для файлов в ZIP
  let options = FileOptions::default()
    .compression_method(zip::CompressionMethod::Stored) // или zip::CompressionMethod::Deflated
    .unix_permissions(0o755) // Устанавливаем права доступа (если необходимо)
    .large_file(false); // Может изменить

  for file in files {
    let mut f = File::open(file)?;
    zip_writer.start_file(file, options)?;

    // Копируем содержимое файла в ZIP
    io::copy(&mut f, &mut zip_writer)?;
  }

  zip_writer.finish()?;
  Ok(())
}

fn extract_zip(zip_path: &str, output_dir: &str) -> io::Result<()> {
  // Открываем ZIP-файл
  let zip_file = File::open(zip_path)?;
  let mut archive = ZipArchive::new(zip_file)?;

  // Создаем выходную директорию, если она не существует
  std::fs::create_dir_all(output_dir)?;

  // Проходим по всем файлам в архиве
  for i in 0..archive.len() {
    let mut file = archive.by_index(i)?;
    let out_path = Path::new(output_dir).join(file.name());

    // Проверяем, является ли элемент директорией
    if file.name().ends_with('/') {
      std::fs::create_dir_all(&out_path)?;
    } else {
      // Создаем родительскую директорию для файла
      if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent)?;
      }

      // Записываем файл на диск
      let mut output_file = File::create(&out_path)?;
      io::copy(&mut file, &mut output_file)?;
    }
  }

  Ok(())
}