extern crate serde_json;
extern crate serde;

mod utils;
mod system;
mod user;
mod cli;

use crate::utils::utilhuffman::{compress, write_to_file, decompress, read_from_file};

fn main() -> std::io::Result<()> {
  let input_text = "this is an example for huffman encoding";
  
  // Сжатие текста
  let (compressed_data, codes) = compress(input_text);
  
  // Запись в файл
  write_to_file("compressed.txt", &compressed_data, &codes)?;

  // Чтение из файла
  let (loaded_codes, loaded_compressed_data) = read_from_file("compressed.txt")?;

  // Распаковка данных
  let decompressed_text = decompress(&loaded_codes, &loaded_compressed_data);

  println!("Original text: {}", input_text);
  println!("Compressed data: {}", compressed_data);
  println!("Decompressed text: {}", decompressed_text);

  Ok(())
}
