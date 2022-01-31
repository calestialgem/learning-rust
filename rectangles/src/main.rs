#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn fits_in(&self, outer: &Rectangle) -> bool {
        self.width <= outer.width && self.height <= outer.height
    }

    // associated function that is not a method
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle::square(45);

    println!(
        "{:?} {} hold {:?}.",
        rect1,
        if rect2.fits_in(&rect1) {
            "can"
        } else {
            "cannot"
        },
        rect2
    );

    println!(
        "{:?} {} hold {:?}.",
        rect1,
        if rect3.fits_in(&rect1) {
            "can"
        } else {
            "cannot"
        },
        rect3
    );
}
