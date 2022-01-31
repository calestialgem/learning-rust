#[derive(Debug)]
enum DaysOfWeek {
    Monday,
    Tuesday,
    Wednesdey,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl DaysOfWeek {
    fn print(&self) {
        println!("Day: {:?}", self);
    }
}

fn main() {
    use DaysOfWeek::*;
    let open_on = vec![Monday, Tuesday, Thursday, Friday, Saturday];
    println!("Our shop is open on:");
    for i in 0..10 {
        match open_on.get(i) {
            Some(d) => d.print(),
            None => println!("The index, {}, is out of bounds!", i),
        }
    }
}
