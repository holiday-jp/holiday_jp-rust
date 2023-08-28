# holiday_jp-rust

[![Build Status](https://travis-ci.org/holiday-jp/holiday_jp-rust.svg?branch=master)](https://travis-ci.org/holiday-jp/holiday_jp-rust)

Get holidays in Japan.


## Install

```
[dependencies]
holiday_jp = { git = "https://github.com/holiday-jp/holiday_jp-rust", branch = "0.2.0" }
time = "0.2.26
```

Already [holiday_jp](https://lib.rs/crates/holiday_jp) crate was reserved.
We need to specify github repository for now.


## Usage

```rust
use holiday_jp::HolidayJp;
use time::{Date, Month};

fn main() {
    let from_date = Date::from_calendar_date(2010, Month::September, 14).unwrap();
    let to_date = Date::from_calendar_date(2010, Month::September, 21).unwrap();
    let holidays = HolidayJp::between(from_date, to_date); // 敬老の日

    let date = Date::from_calendar_date(2016, Month::August, 11).unwrap(); // true
}
```


## Difference of original [holiday_jp](https://lib.rs/crates/holiday_jp) crate

- Original crate fetchs holidays from [Google Calendar](https://github.com/atsushi130/holiday-jp/blob/master/src/holiday_jp/holiday_service.rs#L49-L53). This crate fetchs holidays from [Yaml file](https://github.com/holiday-jp/holiday_jp/blob/master/holidays.yml). It uses [Japanese holiday datasets](https://github.com/holiday-jp/holiday_jp).
- Original crate returns `true` if [Saturday and Sunday](https://github.com/atsushi130/holiday-jp/blob/master/src/holiday_jp/holiday_service.rs#L25-L28). This crate returns `false`.
- This crate has `HolidayJp::is_holiday` and `HolidayJp::between` methods. Those are same method names with [holiday-jp_ruby](https://github.com/holiday-jp/holiday_jp-ruby).
