use chrono::{Datelike, Timelike, Utc};
pub trait Calendar {
    fn jd_num(&self) -> u32;
    fn jd_to_date(&self, jd: u32) -> Date;
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
    date: Date,
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
            date: date,
        }
    }
}

impl Calendar for EthiopianCalendar {
    fn jd_num(&self) -> u32 {
        // https://www.geez.org/Calendars/
        self.j_offset
            + 365 * (self.date.year - 1)
            + (self.date.year / 4)
            + 30 * (self.date.month - 1)
            + self.date.day
            - 1
    }
    fn jd_to_date(&self, jd: u32) -> Date {
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
    fn jd_num(&self) -> u32 {
        // https://www.hermetic.ch/cal_stud/jdn.htm
        (1461 * (self.date.year + 4800 + (self.date.month - 14) / 12)) / 4
            + (367 * (self.date.month - 2 - 12 * ((self.date.month - 14) / 12))) / 12
            - (3 * ((self.date.year + 4900 + (self.date.month - 14) / 12) / 100)) / 4
            + self.date.day
            - 32075
    }
    fn jd_to_date(&self, jd: u32) -> Date {
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
