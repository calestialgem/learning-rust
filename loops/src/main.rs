fn main() {
    let result = {
        let mut counter = 0;
        let mut accumulator = 0;
        while counter != 10 {
            counter += 1;
            accumulator += counter;
        }
        accumulator
    };

    println!("The result is: {}", result);
}
