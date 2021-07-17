#[derive(Debug)]
pub enum Res<T, E> {
  Thing(T),
  Error(E),
}

fn divide(a: i32, b: i32) -> Res<i32, String> {
  if b == 0 {
    return Res::Error("Cannot divide by zero".to_string());
  }
  return Res::Thing(a / b);
}

fn main() {
  let res = divide(100, 10);
  let err = divide(10, 0);

  // Best for multiple cases
  match res {
    Res::Thing(v) => println!("using match: value is {}", v),
    _ => {} // do nothing with the error
  }

  // Best for a single case
  if let Res::Thing(v) = res {
    println!("using if let: value is {}", v)
  }

  println!("res {:?}, err {:?}", res, err);
}
