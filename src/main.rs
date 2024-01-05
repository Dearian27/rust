use std::io;
fn main() {
  let mut input = String::new();
  println!("Let me guess your name");
  println!("Enter your name: ");
  io::stdin().read_line(&mut input).expect("Something went wrong");
  println!("Your name is {input}");
}