fn main() {
    let a = [3, -1, 55, 17, -33];
    let result = {
        let mut accumulator = 0;
        for value in a {
            accumulator += value;
        }
        accumulator
    };

    println!("The result is: {}", result);
}
