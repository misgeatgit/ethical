pub mod Ethical {
    use chrono::{DateTime, Datelike, NaiveDate, TimeZone, Utc};
    use std::fmt::{self, Display, Formatter};
    pub trait Calendar {
        fn jdn(&self) -> u32;
        fn jdn_to_date(&self, jd: u32) -> Date;
        fn month_name(month_n: u32) -> String;
        fn day_name(day_n: u32) -> String;
    }

    #[derive(Debug, Clone)]
    pub struct Date {
        month_name: String,
        day_name: String,
        month: u32,
        year: u32,
        day: u32,
    }

    pub struct EthiopianCalendar {
        date: Date,
    }
    struct GregorianCalendar {
        date: Date,
    }

    impl EthiopianCalendar {
        const J_OFFSET: u32 = 1723856;

        pub fn new() -> Self {
            let now = Utc::now();
            let (_, year) = now.year_ce();
            let date = Date {
                month_name: GregorianCalendar::month_name(now.month()),
                day_name: now.weekday().to_string(),
                month: now.month(),
                day: now.day(),
                year: year,
            };

            let s = Self { date };
            let gc = GregorianCalendar::new(None);
            let date = s.jdn_to_date(gc.jdn());

            Self { date }
        }

        pub fn today(&self) -> Date {
            self.date.clone()
        }
    }

    impl Calendar for EthiopianCalendar {
        fn month_name(month_n: u32) -> String {
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

        fn day_name(day_n: u32) -> String {
            match day_n {
                1 => String::from("ሰኞ"),
                2 => String::from("ማክሰኞ"),
                3 => String::from("ረቡዕ"),
                4 => String::from("ኀሙስ"),
                5 => String::from("ዐርብ"),
                6 => String::from("ቅዳሜ"),
                7 => String::from("እሁድ"),
                _ => panic!("Day number cannot exceed 7."),
            }
        }

        fn jdn(&self) -> u32 {
            // https://www.geez.org/Calendars/
            EthiopianCalendar::J_OFFSET
                + 365 * (self.date.year - 1)
                + (self.date.year / 4)
                + 30 * (self.date.month - 1)
                + self.date.day
                - 1
        }

        fn jdn_to_date(&self, jd: u32) -> Date {
            let jd = jd as i32;
            let offset = EthiopianCalendar::J_OFFSET as i32;
            let r = (jd - offset) % 1461;
            let n = (r % 365) + 365 * (r / 1460);
            let y = 4 * ((jd - offset) / 1461) + (r / 365) - (r / 1460);
            let m = (n / 30) + 1;
            let d = (n % 30) + 1;

            let greg_dt = GregorianCalendar::new(None).jdn_to_date(jd as u32);
            let dt = Utc
                .with_ymd_and_hms(greg_dt.year as i32, greg_dt.month, greg_dt.day, 0, 0, 0)
                .unwrap();

            Date {
                month_name: EthiopianCalendar::month_name(m as u32),
                day_name: EthiopianCalendar::day_name(dt.weekday().number_from_monday()),
                month: m as u32,
                year: y as u32,
                day: d as u32,
            }
        }
    }

    impl Display for Date {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(
                f,
                "{} {} {} {}",
                self.day_name, self.month_name, self.day, self.year
            )
        }
    }

    impl GregorianCalendar {
        fn new(date: Option<DateTime<Utc>>) -> Self {
            let now = date.unwrap_or(Utc::now());
            let (_, year) = now.year_ce();
            let date = Date {
                month_name: GregorianCalendar::month_name(now.month()),
                day_name: now.weekday().to_string(),
                month: now.month(),
                day: now.day(),
                year: year,
            };
            Self { date: date }
        }
    }

    impl Calendar for GregorianCalendar {
        fn month_name(month_n: u32) -> String {
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
                _ => panic!("Month number cannot exceed 12."),
            }
        }

        fn day_name(day_n: u32) -> String {
            match day_n {
                1 => String::from("Moday"),
                2 => String::from("Tuesday"),
                3 => String::from("Wednesday"),
                4 => String::from("Thursday"),
                5 => String::from("Friday"),
                6 => String::from("Saturday"),
                7 => String::from("Suday"),
                _ => panic!("Day number cannot exceed 7."),
            }
        }

        fn jdn(&self) -> u32 {
            // https://www.hermetic.ch/cal_stud/jdn.htm
            let m = self.date.month as i32;
            let y = self.date.year as i32;
            let d = self.date.day as i32;
            let r = (1461 * (y + 4800 + (m - 14) / 12)) / 4
                + (367 * (m - 2 - 12 * ((m - 14) / 12))) / 12
                - (3 * ((y + 4900 + (d - 14) / 12) / 100)) / 4
                + d
                - 32075;
            r as u32
        }

        fn jdn_to_date(&self, jd: u32) -> Date {
            let jd = jd as i32;
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
            let dt = Utc
                .with_ymd_and_hms(y, m as u32, d as u32, 0, 0, 0)
                .unwrap();

            Date {
                month_name: GregorianCalendar::month_name(m as u32),
                day_name: GregorianCalendar::day_name(dt.weekday().number_from_monday()),
                month: m as u32,
                year: y as u32,
                day: d as u32,
            }
        }
    }
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gregorian_jdn() {
        let dt = NaiveDate::from_ymd_opt(2023, 1, 6)
            .unwrap()
            .and_hms_milli_opt(0, 0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap();
        let greg_cal = GregorianCalendar::new(Some(dt));
        assert_eq!(greg_cal.jdn(), 2459951);
    }

    #[test]
    fn test_greg_jdn_to_eth_cal() {
        let dt = NaiveDate::from_ymd_opt(2023, 1, 6)
            .unwrap()
            .and_hms_milli_opt(0, 0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap();
        let greg_cal = GregorianCalendar::new(Some(dt));
        let eth_date = EthiopianCalendar::new().jdn_to_date(greg_cal.jdn());
        assert_eq!(eth_date.year, 2015);
        assert_eq!(eth_date.month, 4);
        assert_eq!(eth_date.day, 28);
        assert_eq!(eth_date.day_name, "ዐርብ");
        assert_eq!(eth_date.month_name, "ታኅሣሥ");
    }
}
}
