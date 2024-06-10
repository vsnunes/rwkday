//! Game mode where the player tries to guess the weekday of a date
//! Based on https://gmmentalgym.blogspot.com/2011/03/day-of-week-for-any-date-revised.html
use core::fmt;

use crate::date::Date;
use rand::Rng;

// Storing Weekday using the mnemonic
pub enum Weekday {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl fmt::Display for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Weekday::Sunday => write!(f, "SUNday=NONEday"),
            Weekday::Monday => write!(f, "MONday=ONEday"),
            Weekday::Tuesday => write!(f, "TWOSday"),
            Weekday::Wednesday => write!(f, "Three fingers look like a W"),
            Weekday::Thursday => write!(f, "FOURSday"),
            Weekday::Friday => write!(f, "FIVEday"),
            Weekday::Saturday => write!(f, "SIXturday"),
        }
    }
}

impl From<crate::date::Weekday> for Weekday {
    fn from(date: crate::date::Weekday) -> Self {
        match date {
            crate::date::Weekday::Monday => Weekday::Monday,
            crate::date::Weekday::Tuesday => Weekday::Tuesday,
            crate::date::Weekday::Wednesday => Weekday::Wednesday,
            crate::date::Weekday::Thursday => Weekday::Thursday,
            crate::date::Weekday::Friday => Weekday::Friday,
            crate::date::Weekday::Saturday => Weekday::Saturday,
            crate::date::Weekday::Sunday => Weekday::Sunday,
        }
    }
}

impl From<u8> for Weekday {
    fn from(weekday: u8) -> Self {
        match weekday {
            0 => Weekday::Sunday,
            1 => Weekday::Monday,
            2 => Weekday::Tuesday,
            3 => Weekday::Wednesday,
            4 => Weekday::Thursday,
            5 => Weekday::Friday,
            6 => Weekday::Saturday,
            _ => panic!("Invalid month"),
        }
    }
}

impl Weekday {
    pub fn as_number(&self) -> u8 {
        match self {
            Weekday::Sunday => 0,
            Weekday::Monday => 1,
            Weekday::Tuesday => 2,
            Weekday::Wednesday => 3,
            Weekday::Thursday => 4,
            Weekday::Friday => 5,
            Weekday::Saturday => 6,
        }
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Month::January => write!(f, "WINTER has 6 letters"),
            Month::February => write!(f, "February is 2nd month"),
            Month::March => write!(f, "March 2 the beat."),
            Month::April => write!(f, "APRIL has 5 letters (& FOOLS!)"),
            Month::May => write!(f, "MAY-0"),
            Month::June => write!(f, "June BUG (BUG has 3 letters)"),
            Month::July => write!(f, "FIVERworks"),
            Month::August => write!(f, "A-1 Steak Sauce at picnic"),
            Month::September => write!(f, "FALL has 4 letters"),
            Month::October => write!(f, "SIX or treat!"),
            Month::November => write!(f, "2 legs on 2rkey"),
            Month::December => write!(f, "LAST (or XMAS) has 4 letters"),
        }
    }
}

impl From<u8> for Month {
    fn from(month: u8) -> Self {
        match month {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => panic!("Invalid month"),
        }
    }
}

impl Month {
    pub fn as_number(&self, leap_year: bool) -> u8 {
        match self {
            Month::January => {
                if leap_year {
                    5
                } else {
                    6
                }
            }
            Month::February => {
                if leap_year {
                    1
                } else {
                    2
                }
            }
            Month::March => 2,
            Month::April => 5,
            Month::May => 0,
            Month::June => 3,
            Month::July => 5,
            Month::August => 1,
            Month::September => 4,
            Month::October => 6,
            Month::November => 2,
            Month::December => 4,
        }
    }
}

pub fn generate_random_date(year_interval: (u16, u16)) -> Date {
    let year = rand::thread_rng().gen_range(year_interval.0..=year_interval.1);
    let month = rand::thread_rng().gen_range(1..=12);
    let day = rand::thread_rng().gen_range(1..=Date::create_month(year, month).month_length());

    Date::create_date(year, month, day)
}

pub fn display_tips(date: &Date, show_solution: bool) {
    println!("Formula: Month Code + Day + Year Code = Day of Week");

    let mut year_code: u16 = 0;

    if let 2000..=2003 = date.year {
        println!("Begining of the century:");
        for year in 2000..=2003 {
            println!("{} = {}", year, year - 2000);
        }

        year_code = date.year - 2000;
    }

    let month = Month::from(date.month);
    let month_code = month.as_number(date.is_leap_year());

    println!(
        "The code for {} is {}, therefore {}.",
        date.month, month, month_code
    );

    println!("{} + {} + {} = ?", month_code, date.day, year_code);

    if show_solution {
        let sum = month_code + date.day + year_code as u8;
        println!("{} therefore {}", sum, Weekday::from(sum));
    }
}
