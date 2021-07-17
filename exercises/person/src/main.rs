#[derive(Debug)]
struct Person {
  name: String,
  age: i32,
  children: i32,
  favourite_colour: Colour,
}

impl Person {
  pub fn print(self) -> String {
    format!(
      "name: {} age: {} children {}",
      self.name, self.age, self.children
    )
  }
}
#[derive(Debug)]
pub enum Colour {
  Red(String),
  Green,
  Blue,
}

fn main() {
  let p = Person {
    name: "name".to_string(),
    age: 99,
    children: 2,
    favourite_colour: Colour::Green,
  };

  let colour = Colour::Red("hello".to_string());

  match colour {
    Colour::Red(s) => println!("It's red! {}", s),
    Colour::Green => println!("It's green!"),
    Colour::Blue => println!("It's blue!"),
  }

  println!("Debug: {:?}", p);
  println!("Printing Person: {}", p.print());
}
