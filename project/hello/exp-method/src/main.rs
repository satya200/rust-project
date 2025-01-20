// This file contains sample code of using methode which is
// derived from struct
#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    let rect1 = Rectangle {
        width: 20,
        height: 40,
    };
    let rect2 = Rectangle {
        width: 40,
        height: 60,
    };
    println!("The area of rectangle is {}", rect.area());
    println!("The area of rectangle is can_hold {}", rect.can_hold(&rect1));
    println!("The area of rectangle is can_hold {}", rect.can_hold(&rect2));
}
