fn main() {
    let y = {
        let x = five();
        plus_one(x)
    };
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
