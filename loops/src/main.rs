fn main() {
    let result = {
        let mut counter = 0;
        let mut accumulator = 0;
        loop {
            counter += 1;
            accumulator += counter;
            if counter == 10 {
                break accumulator;
            }
        }
    };

    println!("The result is: {}", result);
}
