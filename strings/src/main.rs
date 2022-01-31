fn main() {
    let first_name = String::from("Cem");
    let second_name = String::from("Geçgel");
    let gender = String::from("Male");
    let nationality = String::from("Turkish");
    let birth_day = 22;
    let birth_month = String::from("September");
    let birth_year = 2000;
    let identity = format!(
        "{} {}\n [*] Birth: {} {} {}\n [*] Gender: {}\n [*] Nationality: {}",
        first_name,
        second_name.to_uppercase(),
        birth_day,
        birth_month,
        birth_year,
        gender,
        nationality
    );
    println!("{}", identity);
}
