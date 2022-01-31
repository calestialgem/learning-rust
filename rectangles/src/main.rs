#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// implementation block
impl Rectangle {
    fn area(&self) -> u32 {
        // automatic dereferencing
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle {:?} is {} square pixels.",
        rect1,
        rect1.area() // automatic referencing
    );
}
