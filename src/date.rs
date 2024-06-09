//! Computes the Weekday of a date
use std::fmt;

pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl fmt::Display for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Weekday::Monday => write!(f, "Monday"),
            Weekday::Tuesday => write!(f, "Tuesday"),
            Weekday::Wednesday => write!(f, "Wednesday"),
            Weekday::Thursday => write!(f, "Thursday"),
            Weekday::Friday => write!(f, "Friday"),
            Weekday::Saturday => write!(f, "Saturday"),
            Weekday::Sunday => write!(f, "Sunday"),
        }
    }
}

impl Date {
    pub fn create_date(year: u16, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }
    pub fn weekday(&self) -> Weekday {
        let month: f64 = if self.month < 3 {
            12.0 + self.month as f64
        } else {
            self.month as f64
        };

        let month = ((13.0 * (month + 1.0)) / 5.0).floor();
        let year = self.year as f64 % 100.0;
        let zero_based_century = (self.year as f64 / 100.0).floor();

        let weekday = (self.day as f64
            + month
            + year
            + (year / 4.0).floor()
            + (zero_based_century / 4.0).floor()
            - 2.0 * zero_based_century)
            % 7.0;

        let weekday = ((weekday + 5.0) % 7.0) + 1.0;

        match weekday {
            1.0 => Weekday::Monday,
            2.0 => Weekday::Tuesday,
            3.0 => Weekday::Wednesday,
            4.0 => Weekday::Thursday,
            5.0 => Weekday::Friday,
            6.0 => Weekday::Saturday,
            7.0 => Weekday::Sunday,
            _ => panic!("Invalid weekday"),
        }
    }
}
