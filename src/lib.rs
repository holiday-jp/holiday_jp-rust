#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate yaml_rust;

pub mod holiday_jp;

pub use self::holiday_jp::holiday_jp::HolidayJp;

#[cfg(test)]
mod tests {
    use super::HolidayJp;
    use chrono::{Local, TimeZone};

    #[test]
    fn between() {
        let holidays = HolidayJp::between(Local.ymd(2010, 9, 14), Local.ymd(2010, 9, 21));

        assert_eq!(holidays.first().unwrap().name, "敬老の日");
    }

    #[test]
    fn is_holiday() {
        assert!(HolidayJp::is_holiday(Local.ymd(2016, 8, 11)));
    }
}
