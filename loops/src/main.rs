fn main() {
    let a = [3, -1, 55, 17, -33];
    let result = {
        let mut counter = 0;
        let mut accumulator = 0;
        while counter < 5 {
            accumulator += a[counter];
            counter += 1;
        }
        accumulator
    };

    println!("The result is: {}", result);
}
