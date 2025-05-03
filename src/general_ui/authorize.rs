use crate::user::userjsonstore::{JsonStore, JsonStoreTrait};
use crate::user::user::UserLocale;

const USER_DATA_FILE: &str = "userdata.json";

// Структура для вида авторизации пользователя
#[derive(PartialEq)]
pub enum AuthorizeEnum {
  Register,
  Login
}

// Функция для авторизации любым видом UI
pub fn authorize(name: String, email: String, chance: AuthorizeEnum) {
  let mut jsondata = JsonStore::create(USER_DATA_FILE).unwrap();
  jsondata.add_person(
    String::from("userlocale"),
    if chance == AuthorizeEnum::Register { UserLocale::register(name, email) }
    else { UserLocale::login(name, email) }
  );
  jsondata.save(USER_DATA_FILE).unwrap();
}