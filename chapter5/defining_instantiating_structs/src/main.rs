struct User {
    username:  String,
    email: String,
    sign_in_count: u64,
    active: bool,       
   }

fn main() {
   let mut user1 = User {
       email: String::from("eriksson.tomas@gmail.com"),
       username: String::from("Dowie"),
       active: true,
       sign_in_count: 1,
   }; 

   user1.email = "lalal@gmail.com".to_string();

   println!("{}", user1.email);
}
