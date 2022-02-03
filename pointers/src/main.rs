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
    // Here we have a real string. To test the deref coercion we box it. The
    // compiler makes the conversion to &String as we call the function with a
    // &MyBox<String>. This is called implicit deref coercion.
    let real = "Rust";
    let plain = String::from(real);
    let boxed = MyBox::new(plain.clone());
    hello(&plain); // 'normal' call
    hello(&boxed); // implicit deref coercion
    hello(boxed.deref()); // the explicit version of above

    // The String is actually also converted to &str.
    hello(real); // real normal call
    hello(boxed.deref().deref()); // fully explicit version
                                  // &MyBox<String> -> &String -> &str

    hello(&(*boxed)[..]); // or without deref()
                          // the boxed is moved out here
                          // for some reason I don't understand

    // In the end, the compiler calls deref as much as necessary in
    // compilation, which is a lot simpler.
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
