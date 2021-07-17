#[derive(Debug, Clone)]
struct Person {
    name: String, // String doesn't implement copy so Person can't implement copy
    age: i32,
}

// can derive Copy because members x and y implement copy too
// Copy just means "clone by default"
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

fn main() {
    let mut x = 10;
    let y = x;
    x += 1;
    // This works because i32 implements "Copy"
    println!("x: {}, y: {}", x, y);

    let p = Person {
        name: "some name".to_string(),
        age: 99,
    };

    // Deep copy
    let mut p2 = p.clone();
    p2.name.push_str(" changed");

    println!("p: {:?}, p2: {:?}", p, p2);
    let point = Point::new(1, 2);
    let mut point2 = point;
    point2.x = 5;
    point2.y = 6;
    println!("point: {:?}, point2: {:?}", point, point2);
}
