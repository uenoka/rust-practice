fn main() {
    // this is very long
    // let val = Some(3); // when use this three is print
    let val = Some(0u8);

    match val {
        Some(3) => println!("three"),
        _ => (),
    }

    // you can write like this
    if let Some(3) = val {
        println!("three");
    }
}
