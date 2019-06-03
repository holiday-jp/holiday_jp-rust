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
