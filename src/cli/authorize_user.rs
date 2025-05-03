use crate::user::authorize::{AuthorizeEnum, authorize};
use crate::utils::utilio::input;


pub fn authorize_cli() {
  'center: loop {
    let chance = input::<u8>("Choose:\n1.Register\n2.Log in\n");
    match chance {
      1 | 2 => {
        let name = input::<String>("Enter your username: ");
        let email = input::<String>("Enter email: ");
        
        authorize(name, email, 
          if chance == 1 {AuthorizeEnum::Register}
          else {AuthorizeEnum::Login}
        );

        break 'center;
      },
      _ => eprintln!("Incorrect data!\n")
    }
  }
}