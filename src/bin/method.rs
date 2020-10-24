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
}

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 10,
    };
    println!("{:?}", rect);
    println!("{}", rect.area());
}
