use std::io;
fn main() {
  let mut input = String::new(); 
  io::stdin().read_line(&mut input).expect("Error reading line");
  let int_input: i64 = input.trim().parse().unwrap();

  
  println!("{}", int_input + 4);
}