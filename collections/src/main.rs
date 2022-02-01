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

fn main() {
    use std::collections::HashMap;
    use DaysOfWeek::*;
    let mut work_time = HashMap::new();
    work_time.insert(
        String::from("Full-Time"),
        vec![Monday, Tuesday, Thursday, Friday],
    );
    work_time.insert(String::from("Half-Time"), vec![Saturday]);
    work_time.insert(String::from("Never"), vec![Wednesdey, Sunday]);
    println!("Working full-time in: ");
    if let Some(days) = work_time.get(&String::from("Full-Time")) {
        for d in days {
            println!(" [*] {:?}", d);
        }
    }
}
