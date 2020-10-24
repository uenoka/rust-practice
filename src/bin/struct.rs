struct User {
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
    println!("{}", user1.username);

    // this code is error because user1 is not mutable
    // user1.email = String::from("test@example.com");

    // if you want to be mutable, use mut for user variable
    // you can't use mut for values
    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}", user2.email);
    user2.email = String::from("anotheremail@example.com");
    println!("{}", user2.email);

    // easy to write like this
    let build_user = build_user(
        String::from("tanaka_taro@example.com"),
        String::from("tanaka taro"),
    );
    println!("{}", build_user.email);

    // if you want to use other instance value , you can write like this
    let user3 = User {
        email: String::from(""),
        username: String::from(""),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    // and like this
    let user4 = User {
        email: String::from(""),
        username: String::from(""),
        ..user1
    };

    // can I write like this ??
    let user5 = User { ..user1 };
    println!("{}", user5.email);
    // this code is error because user1 is moved to user5
    // println!("{}", user1.email);
}
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
