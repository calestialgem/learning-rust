fn main() {
    let s1 = String::from("hello");
    let (len, s2) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

// Tuples can be used for returning both the result and the ownership
fn calculate_length(s: String) -> (usize, String) {
    (s.len(), s)
}
