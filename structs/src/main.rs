fn main() {
    struct AlwaysEqual; // unit-like struct
    struct AlwaysNotEqual(); // which is a special tuple struct
    let subject = AlwaysEqual;
    let subject = AlwaysNotEqual();
}
