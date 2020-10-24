fn main() {
    let width = 30;
    let height = 50;
    println!("area is {}", area(width, height));

    let rect = (width, height);
    println!("area is {}", area2(rect));

    let rectangle = Rectangle { width, height };
    println!("{:#?}", rectangle);
    println!("area is {}", area3(&rectangle));
}
// this code can run . but we cant know arguments are related by this definition.
// we can improve this by tuple
fn area(width: u32, height: u32) -> u32 {
    width * height
}

// this look like much better .
// but we need to know 0 is width, 1 is height , its too dificult
// so we need to improve by struct
fn area2(dimentions: (u32, u32)) -> u32 {
    dimentions.0 * dimentions.1
}

// you can use println!("{:?}",Rectangle) by adding this #[derive(Debug)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
