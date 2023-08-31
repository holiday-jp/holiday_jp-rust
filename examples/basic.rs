use time::{Date, Month};

use holiday_jp::HolidayJp;

fn main() {
    let from_date = Date::from_calendar_date(2023, Month::September, 14).unwrap();
    let to_date = Date::from_calendar_date(2023, Month::December, 31).unwrap();
    let holidays = HolidayJp::between(from_date, to_date);
    println!("Holiday(s) between {} and {}:", from_date, to_date);
    holidays
        .iter()
        .for_each(|holiday| println!("- {}: {}", holiday.date, holiday.name));

    let date = Date::from_calendar_date(2023, Month::August, 11).unwrap();
    let is_holiday = HolidayJp::is_holiday(date);
    let holidays = HolidayJp::between(date, date);
    println!(
        "Is {} holiday?: {} ({})",
        date,
        is_holiday,
        match is_holiday {
            true => &holidays.first().unwrap().name,
            false => "N/A",
        }
    );
}
