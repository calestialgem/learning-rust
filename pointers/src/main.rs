struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 534_500_000;
    let y = MyBox::new(x);
    println!(
        "Value 0x{:08x} in Address {:016p} or with Deref {:016p}",
        *y, // short of *(y.deref())
        &y, // short of y.deref()
        y.deref()
    );
}
