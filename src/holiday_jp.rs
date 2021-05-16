use super::Holiday;
use super::Holidays;
use time::Date;

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
    use super::HolidayJp;
    use time::Date;

    #[test]
    fn between() {
        let from_date = Date::try_from_ymd(2010, 9, 14).unwrap();
        let to_date = Date::try_from_ymd(2010, 9, 21).unwrap();
        let holidays = HolidayJp::between(from_date, to_date);

        assert_eq!(holidays.first().unwrap().name, "敬老の日");
    }

    #[test]
    fn is_holiday() {
        let date = Date::try_from_ymd(2016, 8, 11).unwrap();

        assert!(HolidayJp::is_holiday(date));
    }
}
