use std::io;
fn main() {
  let mut empty_vector: Vec<i32> = Vec::new();
  let mut initalized_vector: Vec<i32> = vec![1, 3, 4, 5, 1];

  initalized_vector.push(4);
  // let second = initalized_vector.push

  if (false || true) && true && (false || true) {
    println!("Hello, world!");
  }

  println!("{}", initalized_vector[initalized_vector.len() - 1]);
}