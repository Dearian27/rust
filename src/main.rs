use std::io;
fn main() {

  fn solution(num: i32) -> i32 {
    if num < 0 {
      return 0;
    }
    let mut sum: i32 = 0;
    for i in 0..num {
      if i % 3 == 0 || i % 5 == 0 {
        sum += i;
      }
    }
    return sum; 
  }

  println!("{}", solution(10));
}