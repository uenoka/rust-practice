#[derive(Debug)]
enum State {
    NewYork,
    Alaska,
}

enum Coin {
    Penny,
    Nicel,
    Dime,
    Quarter(State),
}
fn main() {
    let coin = Coin::Quarter(State::Alaska);
    let value = value_in_cents(coin);
    println!("{}", value);
}
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("lucky penny");
            1
        }
        Coin::Nicel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
        _ => 100,
    }
}
