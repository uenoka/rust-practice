fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    // you cant write like this , because Option<T> is not T.
    // println!("{}", some_number);
    // println!("{}", some_string);

    // you can write like this
    println!("{:?}", some_number);
    println!("{:?}", some_string);
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // this is error because Option<T> is not T.
    // let sum = x + y;
}
