fn main() {
  let mut n = 0;
  loop {
    if n >= 10 {
      break;
    }
    println!("loop n = {}", n);
    n += 1;
  }
  println!("Loop: All done!");
  n = 0;
  while n <= 10 {
    println!("while n = {}", n);
    n += 1;
  }
  let range = 0..10;
  for i in range {
    println!("for..in i = {}", i);
  }
  println!("for..in: All done!");
}
