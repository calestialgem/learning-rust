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
        println!(" [*] {:?}", self);
    }
}

fn main() {
    use DaysOfWeek::*;
    let open_on = vec![Monday, Tuesday, Thursday, Friday, Saturday];
    println!("Our shop is open on:");
    for d in &open_on {
        d.print();
    }
}
