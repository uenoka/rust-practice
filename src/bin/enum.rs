// define enum
enum IpAddrKind {
    v4,
    v6,
}
#[derive(Debug)]
enum IpAddr {
    v4(String),
    v6(String),
}

#[derive(Debug)]
enum IpAddr2 {
    v4(u8, u8, u8, u8),
    v6(String),
}
fn main() {
    // instantiate
    let four = IpAddr2::v4(127, 0, 0, 1);
    let six = IpAddr::v6(String::from("::1"));
    // route(four);
    route(six);
}
// use by function
fn route(ip_type: IpAddr) {
    println!("{:?}", ip_type);
}
