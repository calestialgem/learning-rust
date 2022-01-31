fn main() {
    let tup = (500, 6.4, 1);
    let together = ('C', 'E', 'M', 22, 9, 2000);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    println!(
        "Name: {}{}{} Birthday: {}.{}.{}",
        together.0, together.1, together.2, together.3, together.4, together.5
    );
}
