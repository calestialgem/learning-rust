fn main() {
    // y binds to the result of the block expression
    let y = {
        let x = five();
        // there is not a semicolon here
        // this is the returned expression
        plus_one(x)
    };
    println!("The value of y is: {}", y);
}

// the return type is defined after arrow
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    // the return expression can be given at the end
    x + 1
}
