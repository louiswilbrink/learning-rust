struct User {
    email: &str,
    username: &str,
    active: bool,
    sign_in_count: u64
}

fn main() {
    let user1 = User {
        email: "louis@louis.com",
        username: "louis",
        active: true,
        sign_in_count: 1
    }
}
