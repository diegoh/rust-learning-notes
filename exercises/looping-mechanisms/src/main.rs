struct Stepper {
    curr: i32,
    step: i32,
    max: i32,
}

impl Iterator for Stepper {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.curr >= self.max {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}

fn main() {
    let mut st = Stepper {
        curr: 0,
        step: 3,
        max: 15,
    };
    loop {
        match st.next() {
            Some(v) => println!("loop: {}", v),
            None => break,
        }
    }
    let mut wh = Stepper {
        curr: 3,
        step: 4,
        max: 15,
    };
    while let Some(v) = wh.next() {
        println!("while: {}", v)
    }
    let ll = Stepper {
        curr: 5,
        step: 10,
        max: 50,
    };
    for i in ll {
        println!("for: {}", i)
    }
    println!("for..in: All done!");
}
