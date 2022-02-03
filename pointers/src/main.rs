fn main() {
    let x = 5;
    let y = &x;
    println!("Value 0x{:08x} in Address {:016p}", *y, y);
}
