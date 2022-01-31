fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // the string is invalid: it is moved from
    let x = 5;
    makes_copy(x);
    // the number is still valid
} // the number, then the string, are out of scope, but nothing is dropped

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // the string is dropped here

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
