use std::str::FromStr;
use std::fmt::Debug;

pub fn input<T: FromStr>(message: &str) -> T where <T as FromStr>::Err: Debug {
  eprint!("{}", message);
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).unwrap();
    
  // Обработка возможной ошибки при парсинге
  input.trim().parse().unwrap()
}