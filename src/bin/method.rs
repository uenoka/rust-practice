#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// you can implement area function for Rectangle by using impl (like OOP).
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    // if &self in argument , its Method of struct
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // if & not in argument , its related function, (not Method)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// you can write some imple block.
impl Rectangle {
    fn some_method(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 10,
    };
    println!("{:?}", rect);
    println!("{}", rect.area());

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 40,
        height: 20,
    };

    println!("rect1 can hold rect2? {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3? {}", rect1.can_hold(&rect3));
    let square = Rectangle::square(10);
    println!("{:?}", square);
}
