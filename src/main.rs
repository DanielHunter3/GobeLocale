extern crate serde_json;
extern crate serde;

mod utils;
mod system;
mod user;
mod cli;

fn main() {
  match system::repo::LocaleRepository::create("Charlie".into()) {
    Ok(repo) => {
      println!("Repository created successfully");
      // Use repo as needed
    },
    Err(e) => {
      eprintln!("Failed to create repository: {}", e);
      eprintln!("Current directory: {:?}", std::env::current_dir());
      std::process::exit(1);
    }
  }
}
