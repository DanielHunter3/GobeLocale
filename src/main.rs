extern crate serde_json;
extern crate serde;

mod utils;
mod system;
mod user;
mod cli;

fn main() {
  let b = Box::new(50);
  let p: *const usize = &50;
  assert_eq!(std::mem::size_of_val(&b), std::mem::size_of_val(&p))
}
