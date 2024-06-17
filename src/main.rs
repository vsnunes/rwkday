use clap::Parser;
use std::io;
mod date;
mod game;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Date in the ISO 8601 format: YYYY-MM-DD (Ignored when game mode is on)
    #[arg(short, long, default_value_t = String::from(""))]
    date: String,

    /// Display numeric weekday (1 = Monday .. 7 = Sunday) rather than string representation
    #[arg(short, long, default_value_t = false)]
    number: bool,

    /// Enables game mode
    #[arg(short, long, default_value_t = false)]
    game: bool,

    /// Enables tips for game mode. Ignored unless -g or --game is also set
    #[arg(short, long, default_value_t = false)]
    tips: bool,

    /// Computes year code for given year
    #[arg(short, long, default_value_t = 0)]
    year: u16,
}

fn main() {
    let args = Args::parse();

    if args.year != 0 {
        println!(
            "{}",
            game::year_code(&date::Date {
                year: args.year,
                month: 1,
                day: 1,
            })
        );
        return;
    }

    let parsed_date: Vec<&str> = args.date.split("-").collect();
    let date = date::Date::create_date(
        parsed_date[0].parse().unwrap(),
        parsed_date[1].parse().unwrap(),
        parsed_date[2].parse().unwrap(),
    );

    if args.game {
        let guess_date = date; //game::generate_random_date((1900, 2100));
        let mut user_weekday_guess = String::new();

        println!("Guess the weekday of {guess_date}");

        if args.tips {
            println!("================================");
            game::display_tips(&guess_date, false);
            println!("================================");
        }

        io::stdin().read_line(&mut user_weekday_guess).expect(
            "You need to provide a weekday in numeric format (1 - Monday, ..., 7 - Sunday)",
        );

        let user_weekday_guess: u8 = user_weekday_guess
            .trim()
            .parse()
            .expect("You provided something else than a number :(");
        let correct_weekday = guess_date.weekday();

        if user_weekday_guess == correct_weekday.as_number() {
            println!("Right on! {} is {}", guess_date, correct_weekday);
        } else {
            println!("Not really! {} is {}", guess_date, correct_weekday);
        }
        return;
    }

    if args.number {
        println!("Weekday: {}", date.weekday().as_number());
    } else {
        println!("Weekday: {}", date.weekday());
    }
}
