use chrono::{Datelike, Utc};
use std::collections::HashMap;
pub trait Calendar {
    fn jdn(&self) -> u32;
    fn jdn_to_date(&self, jd: u32) -> Date;
    fn month_name(month_n: i32) -> String;
}

pub struct Date {
    month_name: String,
    day_name: String,
    month: u32,
    year: u32,
    day: u32,
}

pub struct EthiopianCalendar {
    j_offset: u32,
    greg_date: Date,
}
pub struct GregorianCalendar {
    date: Date,
}

impl EthiopianCalendar {
    fn new() -> Self {
        let now = Utc::now();
        let (_, year) = now.year_ce();
        let date = Date {
            month_name: String::from("TODO"),
            day_name: String::from("TODO"),
            month: now.month(),
            day: now.day(),
            year: year,
        };
        Self {
            j_offset: 1723856,
            greg_date: date,
        }
    }
}

impl Calendar for EthiopianCalendar {
    fn month_name(month_n: i32) -> String {
        match month_n {
            1 => String::from("መስከረም"),
            2 => String::from("ጥቅምት"),
            3 => String::from("ኅዳር"),
            4 => String::from("ታኅሣሥ"),
            5 => String::from("ጥር"),
            6 => String::from("የካቲት"),
            7 => String::from("መጋቢት"),
            8 => String::from("ሚያዝያ"),
            9 => String::from("ግንቦት"),
            10 => String::from("ሰኔ"),
            11 => String::from("ሐምሌ"),
            12 => String::from("ነሐሴ"),
            13 => String::from("ጳጉሜን"),
            _ => panic!("Month number cannot exceed 13."),
        }
    }

    fn jdn(&self) -> u32 {
        // https://www.geez.org/Calendars/
        self.j_offset
            + 365 * (self.greg_date.year - 1)
            + (self.greg_date.year / 4)
            + 30 * (self.greg_date.month - 1)
            + self.greg_date.day
            - 1
    }

    fn jdn_to_date(&self, jd: u32) -> Date {
        let r = (jd - self.j_offset) % 1461;
        let n = r % 365 + 365 * r / 1460;
        let y = 4 * (jd - self.j_offset) / 1461 + r / 365 - r / 1460;
        let m = n / 30 + 1;
        let d = n % 30 + 1;

        Date {
            month_name: String::from("TODO"),
            day_name: String::from("TODO"),
            month: m,
            year: y,
            day: d,
        }
    }
}

impl GregorianCalendar {
    fn new() -> Self {
        let now = Utc::now();
        let (_, year) = now.year_ce();
        let date = Date {
            month_name: String::from("TODO"),
            day_name: String::from("TODO"),
            month: now.month(),
            day: now.day(),
            year: year,
        };
        Self { date: date }
    }
}

impl Calendar for GregorianCalendar {
    fn month_name(month_n: i32) -> String {
        match month_n {
            1 => String::from("January"),
            2 => String::from("February"),
            3 => String::from("March"),
            4 => String::from("Apri"),
            5 => String::from("May"),
            6 => String::from("June"),
            7 => String::from("July"),
            8 => String::from("August"),
            9 => String::from("September"),
            10 => String::from("October"),
            11 => String::from("November"),
            12 => String::from("December"),
            _ => panic!("Month number cannot exceed 13."),
        }
    }

    fn jdn(&self) -> u32 {
        // https://www.hermetic.ch/cal_stud/jdn.htm
        (1461 * (self.date.year + 4800 + (self.date.month - 14) / 12)) / 4
            + (367 * (self.date.month - 2 - 12 * ((self.date.month - 14) / 12))) / 12
            - (3 * ((self.date.year + 4900 + (self.date.month - 14) / 12) / 100)) / 4
            + self.date.day
            - 32075
    }

    fn jdn_to_date(&self, jd: u32) -> Date {
        let mut l = jd + 68569;
        let n = (4 * l) / 146097;
        l = l - (146097 * n + 3) / 4;
        let i = (4000 * (l + 1)) / 1461001;
        l = l - (1461 * i) / 4 + 31;
        let j = (80 * l) / 2447;
        let d = l - (2447 * j) / 80;
        l = j / 11;
        let m = j + 2 - (12 * l);
        let y = 100 * (n - 49) + i + l;

        Date {
            month_name: String::from("TODO"),
            day_name: String::from("TODO"),
            month: m,
            year: y,
            day: d,
        }
    }
}
