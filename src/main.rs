extern crate serde_json;
extern crate serde;

mod utils;
mod system;
mod user;
mod cli;
mod general_ui;

use cli::authorize_user;

fn main() {
    authorize_user::authorize_cli();
}
