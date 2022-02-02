use std::collections::HashMap;

fn main() {
    let mut weekly = HashMap::new();
    weekly.insert("Monday", 12);
    weekly.insert("Tuesday", 12);
    weekly.insert("Thursday", 12);
    weekly.insert("Friday", 12);
    weekly.insert("Sunday", 6);
    print_all(&weekly);
    print("Monday", &weekly);
    overwriting_a_value(&mut weekly);
    only_inserting_a_value_if_the_key_has_no_value(&mut weekly);
    updating_a_value_based_on_the_old_value(&mut weekly);
}

fn print(day: &str, weekly: &HashMap<&str, i32>) {
    println!(
        "On {}s, I work {} hours.",
        day,
        weekly
            .get(day)
            .expect("Could not find information on the given day!")
    );
}

fn print_all(weekly: &HashMap<&str, i32>) {
    println!("Weekly working time: ");
    for (d, h) in weekly {
        println!("[*] {}: {}", d, h);
    }
}

fn overwriting_a_value(weekly: &mut HashMap<&str, i32>) {
    weekly.insert("Monday", 17);
    print("Monday", weekly);
}

fn only_inserting_a_value_if_the_key_has_no_value(weekly: &mut HashMap<&str, i32>) {
    for d in [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ] {
        weekly.entry(d).or_insert(0);
    }
    print_all(weekly);
}

fn updating_a_value_based_on_the_old_value(weekly: &mut HashMap<&str, i32>) {
    let extra = {
        let mut extra = HashMap::new();
        extra.insert("Monday", 5);
        extra.insert("Tuesday", 5);
        extra.insert("Wednesday", 12);
        extra.insert("Thursday", 5);
        extra.insert("Friday", 5);
        extra.insert("Saturday", 12);
        extra
    };
    for (day, hours) in extra {
        let weekly_day = weekly.entry(day).or_insert(0);
        *weekly_day += hours;
    }
    print_all(weekly);
}
