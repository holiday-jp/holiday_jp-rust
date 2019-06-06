# holiday_jp-rust

[![Build Status](https://travis-ci.org/camelmasa/holiday_jp-rust.svg?branch=master)](https://travis-ci.org/camelmasa/holiday_jp-rust)

Get holidays in Japan.


## Install

```
[dependencies]
holiday_jp = { git = "https://github.com/camelmasa/holiday_jp-rust", branch = "0.1.0" }
```

Already [holiday_jp](https://lib.rs/crates/holiday_jp) crate was reserved.
We need to specify github repository for now.


## Usage

```rust
use chrono::{Local, TimeZone};
use holiday_jp::HolidayJp;

fn main() {   
    let holidays = HolidayJp::between(Local.ymd(2010, 9, 14), Local.ymd(2010, 9, 21));
    holidays.first().unwrap().name; // 敬老の日
    HolidayJp::is_holiday(Local.ymd(2016, 8, 11)); // true
}
```


## Difference of original [holiday_jp](https://lib.rs/crates/holiday_jp) crate

- Original crate fetchs holidays from [Google Calendar](https://github.com/atsushi130/holiday-jp/blob/master/src/holiday_jp/holiday_service.rs#L49-L53). This crate fetchs holidays from [Yaml file](https://github.com/camelmasa/holiday_jp-rust/blob/master/holidays.yml). It uses [Japanese holiday datasets](https://github.com/holiday-jp/holiday_jp).
- Original crate returns `true` if [Saturday and Sunday](https://github.com/atsushi130/holiday-jp/blob/master/src/holiday_jp/holiday_service.rs#L25-L28). This crate returns `false`.
- This crate has `HolidayJp::is_holiday` and `HolidayJp::between` methods. Those are same method names with [holiday-jp_ruby](https://github.com/holiday-jp/holiday_jp-ruby).
