fn main() {
    let x = 534_500_000;
    let y = Box::new(x);
    println!("Value 0x{:08x} in Address {:016p}", *y, y);
}
