use time::Date;

use super::Holiday;
use super::Holidays;

lazy_static! {
    static ref HOLIDAYS: Holidays = Holidays::new();
}

pub struct HolidayJp {}

impl HolidayJp {
    pub fn between(start: Date, last: Date) -> Vec<&'static Holiday> {
        HOLIDAYS
            .holidays
            .iter()
            .filter(|(_key, value)| start <= value.date && value.date <= last)
            .map(|(_key, value)| value)
            .collect()
    }

    pub fn is_holiday(date: Date) -> bool {
        HOLIDAYS.is_holiday(date)
    }
}

#[cfg(test)]
mod tests {
    use time::{Date, Month};

    use super::HolidayJp;

    #[test]
    fn between() {
        let from_date = Date::from_calendar_date(2010, Month::September, 14).unwrap();
        let to_date = Date::from_calendar_date(2010, Month::September, 21).unwrap();
        let holidays = HolidayJp::between(from_date, to_date);

        assert_eq!(holidays.first().unwrap().name, "敬老の日");
    }

    #[test]
    fn is_holiday() {
        let date = Date::from_calendar_date(2016, Month::August, 11).unwrap();

        assert!(HolidayJp::is_holiday(date));
    }
}
