struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,    // field init shorthand
        username, // field init shorthand
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    let user3 = User {
        email: String::from("anotheremail@example.com"),
        ..user2 // struct update syntax
    };
    println!(
        "User: {} <{}> ({}) {}",
        user3.username,
        user3.email,
        user3.sign_in_count,
        if user3.active { "Online" } else { "Offline" }
    );
}
