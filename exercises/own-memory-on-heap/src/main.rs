// Without box: infinite size linked list, the compiler doesn't know how to calculate the size
// Box = use a pointer, if there is nothing, only take the size of the data pointer
#[derive(Debug)]
pub struct LinkedList<T> {
    data: T,
    next: Option<Box<LinkedList<T>>>,
}
// if type T implements trait AddAssign (+=) it will get these methods
impl<T: std::ops::AddAssign> LinkedList<T> {
    pub fn add_up(&mut self, increment: T) {
        self.data += increment;
    }
}

fn main() {
    // Rust can infer that the type T of LinkedList ll is i32 because the type of property "data" is i32
    let mut ll = LinkedList {
        data: 3,
        next: Some(Box::new(LinkedList {
            data: 5,
            next: None,
        })),
    };

    ll.add_up(5);
    println!("ll {:?}", ll);
    assert_eq!(ll.data, 8);
    // Sometimes we just want a mutable reference to the object rather than changing the object itself
    if let Some(ref mut v) = ll.next {
        v.add_up(10);
        assert_eq!(v.data, 15);
    }

    println!("ll {:?}", ll);
    // a Vector is like a resizeable array, it has:
    // - length
    // - capacity: how much memory have we reserved. On resize, it grabs twice as much memory as it had and copies over the values to the new space and free the old memory
    // - a pointer to the actual data
    let mut v: Vec<String> = Vec::new();
    v.push("hello".to_string());
    println!("v {:?}", v);
    v.push("goodbye".to_string());
    println!("v len: {}, capacity: {}", v.len(), v.capacity());
    let mut v2: Vec<String> = Vec::with_capacity(100);
    println!("v len: {}, capacity: {}", v2.len(), v2.capacity());
    v2.push("hello".to_string());
    v2.push("goodbye".to_string());
    for i in 0..105 {
        v2.push(i.to_string());
    }
    println!("v len: {}, capacity: {}", v2.len(), v2.capacity()); // double capacity so that it can keep roughly constant time

    let v3 = v2;
    println!("v3 {:?}", v3);
}
