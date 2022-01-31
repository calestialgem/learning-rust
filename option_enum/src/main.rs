fn main() {
    let a = Some(8);
    let a = add_seven(a);
    print(a);
    let a = add_seven(None);
    print(a);
}

fn add_seven(a: Option<i32>) -> Option<i32> {
    match a {
        Some(a) => Some(a + 7),
        None => None,
    }
}

fn print(a: Option<i32>) {
    match a {
        Some(a) => println!("The number is {}.", a),
        None => println!("There is no number!"),
    };
}
