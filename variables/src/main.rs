fn main() {
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: [{}, {}, {}, {}, {}]", a[0], a[1], a[2], a[3], a[4]);
    let first = a[0];
    let second = a[1];
    let a = [3; 5];
    println!("Array: [{}, {}, {}, {}, {}]", a[0], a[1], a[2], a[3], a[4]);
}
