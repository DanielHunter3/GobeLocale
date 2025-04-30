extern crate serde_json;
extern crate iced;

mod utils;
mod system;
mod user;

fn main() {
 //let repo = system::repo::LocaleRepository::new("I".to_string(), "Some".to_string());
 let author = user::user::UserLocale::new("I".to_string(), "w".to_string());
}
