struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    user1.email = String::from("anotheremail@example.com");
    println!(
        "User: {} <{}> ({}) {}",
        user1.username,
        user1.email,
        user1.sign_in_count,
        if user1.active { "Online" } else { "Offline" }
    );
}
