struct User {
    username: Name,
    email: Email,
    sign_in_count: u64,
    active: bool,
}
struct Name {
    first_name: String,
    last_name: String,
    middle_name: String,
}
impl Name {
    fn get(&self) -> String {
        format!(
            "{} {} {}",
            self.first_name, self.middle_name, self.last_name
        )
    }
}

struct Email {
    domain: String,
    user_part: String,
}

impl Email {
    fn get(&self) -> String {
        return format!("{}@{}", self.user_part, self.domain);
    }
}
fn main() {
    let mut mail = Email {
        domain: String::from("gmail.com"),
        user_part: String::from("k.ueno.0505"),
    };
    let mut name = Name {
        first_name: String::from("kazuki"),
        last_name: String::from("Ueno"),
        middle_name: String::from(""),
    };
    let mut user = User {
        email: mail,
        username: name,
        active: true,
        sign_in_count: 1,
    };
    println!(
        "name is {}, email is {}, active is {},sign in count is {}",
        user.username.get(),
        user.email.get(),
        user.active,
        user.sign_in_count
    );
    mail = Email {
        domain: String::from("gmail.com"),
        user_part: String::from("ka.ueno"),
    };
    name = Name {
        first_name: String::from("tanaka"),
        last_name: String::from("taro"),
        middle_name: String::from(""),
    };
    user.email = mail;
    user.username = name;
    println!(
        "name is {}, email is {}, active is {},sign in count is {}",
        user.username.get(),
        user.email.get(),
        user.active,
        user.sign_in_count
    );
}
