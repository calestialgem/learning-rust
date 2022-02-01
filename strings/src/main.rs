fn main() {
    let name = String::from("Cem Geçgel");
    println!("String: {}", name);
    println!("Characters:");
    for c in name.chars() {
        println!(" [*] {}", c);
    }
    println!("Bytes:");
    for b in name.bytes() {
        println!(" [*] {}", b);
    }
}
