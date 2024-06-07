use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let raw_date = &args[1];
    let date: Vec<&str> = raw_date.split("/").collect();
    let year: u16 = date[0].parse().unwrap();
    let month: u8 = date[1].parse().unwrap();
    let day: u8 = date[2].parse().unwrap();

    println!("Date: {year}/{month}/{day}");
}
