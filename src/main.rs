use system::repo::LocaleRepository;

extern crate serde_json;
extern crate serde;

mod utils;
mod system;
mod user;
mod cli;

fn main() {
  let x = LocaleRepository::new("David".to_string(), "CLang++".to_string());
  match x.init()  {
    Ok(_) => {},
    Err(_) => {println!("{}", x.get_repo_folder())}
  }
}
