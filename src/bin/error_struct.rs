struct User {
    // this code is error because of missing lifetime specifier
    // username: &str,
    // email: &str,
    // so you need to use String
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}
