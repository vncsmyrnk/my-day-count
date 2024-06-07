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
            Month::February => if is_leap_year(year) { 29 } else { 28 },
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
}

struct Date {
    day: u8,
    month: Month,
    year: u32,
}

impl Date {
    fn days_in_month(&self) -> u8 {
        self.month.days(self.year)
    }
}

fn is_leap_year(year: u32) -> bool {
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
