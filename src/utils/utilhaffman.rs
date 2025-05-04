use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{self, Read, Write};
use std::cmp::Ordering;

#[derive(Debug)]
struct Node {
  char: Option<char>,
  freq: usize,
  left: Option<Box<Node>>,
  right: Option<Box<Node>>,
}

impl Node {
  fn new(char: Option<char>, freq: usize) -> Self {
    Node {
      char,
      freq,
      left: None,
      right: None,
    }
  }
}

// Реализация Ord для использования в BinaryHeap
impl Ord for Node {
  fn cmp(&self, other: &Self) -> Ordering {
    other.freq.cmp(&self.freq) // Обратный порядок для создания минимальной кучи
  }
}

impl PartialOrd for Node {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for Node {
  fn eq(&self, other: &Self) -> bool {
    self.freq == other.freq
  }
}

impl Eq for Node {}

pub fn build_huffman_tree(frequencies: &HashMap<char, usize>) -> Option<Box<Node>> {
  let mut heap = BinaryHeap::new();

  for (&char, &freq) in frequencies.iter() {
    heap.push(Node::new(Some(char), freq));
  }

  while heap.len() > 1 {
    let left = heap.pop().unwrap();
    let right = heap.pop().unwrap();
    let combined_freq = left.freq + right.freq;

    let mut parent = Node::new(None, combined_freq);
    parent.left = Some(Box::new(left));
    parent.right = Some(Box::new(right));

    heap.push(parent);
  }

  heap.pop().map(Box::new)
}

pub fn generate_codes(node: &Node, prefix: String, codes: &mut HashMap<char, String>) {
  if let Some(char) = node.char {
    codes.insert(char, prefix);
  } else {
    if let Some(ref left) = node.left {
      generate_codes(left, format!("{}0", prefix), codes);
    }
    if let Some(ref right) = node.right {
      generate_codes(right, format!("{}1", prefix), codes);
    }
  }
}

pub fn compress(input: &str) -> (String, HashMap<char, String>) {
  // Подсчет частоты символов
  let mut frequencies = HashMap::new();
  for char in input.chars() {
    *frequencies.entry(char).or_insert(0) += 1;
  }

  // Построение дерева Хаффмана
  let root = build_huffman_tree(&frequencies).unwrap();
  
  // Генерация кодов Хаффмана
  let mut codes = HashMap::new();
  generate_codes(&root, String::new(), &mut codes);

  // Сжатие текста
  let compressed_data: String = input.chars().map(|c| codes[&c].clone()).collect();
  
  (compressed_data, codes)
}

pub fn write_to_file(filename: &str, compressed_data: &str, codes: &HashMap<char, String>) -> io::Result<()> {
  let mut file = File::create(filename)?;
  
  // Запись кодов в файл
  for (char, code) in codes.iter() {
    writeln!(file, "{}:{}", char, code)?;
  }
  
  // Запись сжатых данных
  writeln!(file, "DATA:{}", compressed_data)?;
  
  Ok(())
}

pub fn read_from_file(filename: &str) -> io::Result<(HashMap<char, String>, String)> {
  let mut file = File::open(filename)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  let mut codes = HashMap::new();
  let mut compressed_data = String::new();

  for line in contents.lines() {
    if line.starts_with("DATA:") {
      compressed_data = line[5..].to_string();
    } else {
      let parts: Vec<&str> = line.split(':').collect();
      if parts.len() == 2 {
        codes.insert(parts[0].chars().next().unwrap(), parts[1].to_string());
      }
    }
  }

  Ok((codes, compressed_data))
 }
 
pub fn decompress(codes: &HashMap<char, String>, compressed_data: &str) -> String {
  let mut reversed_codes: HashMap<String, char> = HashMap::new();
   
  for (char, code) in codes.iter() {
    reversed_codes.insert(code.clone(), *char);
  }
 
   let mut current_code = String::new();
   let mut decompressed_data = String::new();
 
   for bit in compressed_data.chars() {
    current_code.push(bit);
     
    if let Some(&char) = reversed_codes.get(&current_code) {
      decompressed_data.push(char);
      current_code.clear();
    }
  }
 
  decompressed_data
}