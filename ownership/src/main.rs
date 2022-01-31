fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // s2 is moved out from, it is invalid
} // s3, then s2 are dropped

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
    // the string is moved out to the calling function
} // it is not dropped here as it is invalid

fn takes_and_gives_back(a_string: String) -> String {
    a_string
    // the string is moved out to the calling function
} // it is not dropped here as it is invalid
