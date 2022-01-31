fn main() {
    let mut s1 = String::from("hello");
    change(&mut s1);

    let r1 = &s1;
    let r2 = &s1;
    // Immutable references can exist at the same time.
    println!("{} and {}", r1, r2);

    // Non-Lexical Lifetimes apply here, the mutable reference does not exist
    // together with the immutable ones above. The compiler can tell that those
    // immutable references are not used anymore, they cannot create problems.
    let r3 = &mut s1;
    println!("{}", r3);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
