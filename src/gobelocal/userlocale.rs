

pub struct UserLocale {
  username: String,
  password: String,
  email: String
}

impl UserLocale {
  pub fn register(username: String, 
            password: String, 
            email: String) -> UserLocale 
  {
    UserLocale { username, password, email }
  }

  pub fn login(username: String, password: String) -> UserLocale {
    UserLocale { username, password, email: String::new() }
  }
}