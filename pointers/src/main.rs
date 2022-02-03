enum List<T> {
    Cons(T, List<T>),
    Nil,
}

use List::*;

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
