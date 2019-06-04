use super::holiday::Holiday;
use super::holidays::Holidays;
use chrono::{Date, Local};

lazy_static! {
    static ref HOLIDAYS: Holidays = Holidays::new();
}

pub struct HolidayJp {}

impl HolidayJp {
    pub fn between(start: Date<Local>, last: Date<Local>) -> Vec<&'static Holiday> {
        HOLIDAYS
            .holidays
            .iter()
            .filter(|(_key, value)| start <= value.date && value.date <= last)
            .map(|(_key, value)| value)
            .collect()
    }

    pub fn is_holiday(date: Date<Local>) -> bool {
        HOLIDAYS.is_holiday(date)
    }
}

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
