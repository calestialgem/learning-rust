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
    for days in &work_time {
        print!("Working {} in ", days.0);
        for d in days.1 {
            print!("{:?} ", d);
        }
        println!(".");
    }
}
