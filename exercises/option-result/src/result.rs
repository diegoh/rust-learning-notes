fn divide(a: i32, b: i32) -> Result<i32, String> {
  if b == 0 {
    return Err("Cannot divide by zero".to_string());
  }
  return Ok(a / b);
}

fn main() {
  let res = divide(100, 10);
  let err = divide(10, 0);

  // Best for multiple cases
  match res {
    Ok(v) => println!("using match: value is {}", v),
    _ => {} // do nothing with the error
  }

  // Best for a single case
  if let Ok(v) = res {
    println!("using if let: value is {}", v)
  }

  println!("res {:?}, err {:?}", res, err);
}
