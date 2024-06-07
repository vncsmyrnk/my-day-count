#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Month {
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

impl Month {
    fn days(&self, year: u32) -> u8 {
        match self {
            Month::January => 31,
            Month::February => if is_leap_year(&year) { 29 } else { 28 },
            Month::March => 31,
            Month::April => 30,
            Month::May => 31,
            Month::June => 30,
            Month::July => 31,
            Month::August => 31,
            Month::September => 30,
            Month::October => 31,
            Month::November => 30,
            Month::December => 31,
        }
    }

    fn order(&self) -> u8 {
        match self {
            Month::January => 1,
            Month::February => 2,
            Month::March => 3,
            Month::April => 4,
            Month::May => 5,
            Month::June => 6,
            Month::July => 7,
            Month::August => 8,
            Month::September => 9,
            Month::October => 10,
            Month::November => 11,
            Month::December => 12,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Date {
    day: u8,
    month: Month,
    year: u32,
}

use std::cmp::Ordering;

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.year.cmp(&other.year) {
            Ordering::Equal => match self.month.cmp(&other.month) {
                Ordering::Equal => self.day.cmp(&other.day),
                other => other,
            },
            other => other,
        }
    }
}

impl Date {
    fn days_in_month(&self) -> u8 {
        self.month.days(self.year)
    }
}

fn is_leap_year(year: &u32) -> bool {
    year % 4 == 0
}

fn main() {
    let date = Date {
        day: 8,
        month: Month::February,
        year: 2023
    };
    println!("How many days we have in may? {}", date.days_in_month());
}
