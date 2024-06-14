//! Computes the Weekday of a date
use std::fmt;

pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
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

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{:02}-{:02}", self.year, self.month, self.day)
    }
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

impl Date {
    pub fn create_date(year: u16, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    pub fn create_month(year: u16, month: u8) -> Self {
        Self {
            year,
            month,
            day: 1,
        }
    }

    // Based on RFC 3339 Appendix B
    pub fn weekday(&self) -> Weekday {
        let day = self.day as i16;
        let mut month = self.month as i16;
        let mut year = self.year as i16;

        // adjust months so February is the last one
        month = month - 2;
        if month < 1 {
            month = month + 12;
            year = year - 1;
        }

        let cent = year / 100;
        year = year % 100;

        let weekday = ((26 * month - 2) / 10 + day + year + year / 4 + cent / 4 + 5 * cent) % 7;

        match weekday {
            0 => Weekday::Sunday,
            1 => Weekday::Monday,
            2 => Weekday::Tuesday,
            3 => Weekday::Wednesday,
            4 => Weekday::Thursday,
            5 => Weekday::Friday,
            6 => Weekday::Saturday,
            _ => panic!("Invalid weekday"),
        }
    }

    pub fn is_leap_year(&self) -> bool {
        self.year % 4 == 0 && (self.year % 100 != 0 || self.year % 400 == 0)
    }

    pub fn month_length(&self) -> u8 {
        match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => return 31,
            4 | 6 | 9 | 11 => return 30,
            2 => return if self.is_leap_year() { 29 } else { 28 },
            _ => panic!("Invalid month"),
        }
    }
}
