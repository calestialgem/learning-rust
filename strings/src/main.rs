fn main() {
    let data = "initial contents";
    let mut s = data.to_string();
    s.push_str(" some other contents ile this character: ");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from(" world!");
    println!("{}", s1 + &s2);
}
