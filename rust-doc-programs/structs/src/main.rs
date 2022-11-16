struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn create_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

fn main() {

    let mut user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };

    user.email = String::from("updatedemail@example.com");

    println!("{}, {}, {}, {}", user.email, user.username, user.active, user.sign_in_count);

    let second_user = create_user(String::from("seconduser@example.com"), String::from("secondusername123"));

    println!("{}, {}, {}, {}", second_user.email, second_user.username, second_user.active, second_user.sign_in_count);

}
