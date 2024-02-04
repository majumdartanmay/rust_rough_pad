struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
   let mut user1: User = User {
      active: true,
      username: "asd".to_string(),
      email: String::from("someone@example.com"),
      sign_in_count: 1,
   };
   user1.email = "changedstring".to_string();
   print!("{}", user1.username);
}


