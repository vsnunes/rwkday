use clap::Parser;
mod date;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Date in the ISO 8601 format: YYYY-MM-DD
    date: String,

    /// Display numeric weekday (1 = Monday .. 7 = Sunday) rather than string representation
    #[arg(short, long, default_value_t = false)]
    number: bool,
}

fn main() {
    let args = Args::parse();

    let parsed_date: Vec<&str> = args.date.split("-").collect();
    let date = date::Date::create_date(
        parsed_date[0].parse().unwrap(),
        parsed_date[1].parse().unwrap(),
        parsed_date[2].parse().unwrap(),
    );

    if args.number {
        println!("Weekday: {}", date.weekday().as_number());
    } else {
        println!("Weekday: {}", date.weekday());
    }
}
