use std::env;
mod date;

fn main() {
    let args: Vec<String> = env::args().collect();

    let raw_date = &args[1];
    let parsed_date: Vec<&str> = raw_date.split("/").collect();
    let date = date::Date::create_date(
        parsed_date[0].parse().unwrap(),
        parsed_date[1].parse().unwrap(),
        parsed_date[2].parse().unwrap(),
    );

    println!("Weekday: {}", date.weekday());
}
