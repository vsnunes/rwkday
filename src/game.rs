//! Game mode where the player tries to guess the weekday of a date
use crate::date::Date;
use rand::Rng;

pub fn generate_random_date(year_interval: (u16, u16)) -> Date {
    let year = rand::thread_rng().gen_range(year_interval.0..=year_interval.1);
    let month = rand::thread_rng().gen_range(1..=12);
    let day = rand::thread_rng().gen_range(1..=Date::create_month(year, month).month_length());

    Date::create_date(year, month, day)
}
