use super::holiday::Holiday;
use chrono::{Date, Local, TimeZone};
use std::collections::HashMap;
use yaml_rust::YamlLoader;

pub struct Holidays {
    pub holidays: HashMap<String, Holiday>,
}

impl Holidays {
    pub fn new() -> Self {
        const HOLIDAYS_STRING: &str = include_str!("../../holidays.yml");
        let docs = YamlLoader::load_from_str(HOLIDAYS_STRING).unwrap();

        let mut holidays = HashMap::new();

        for (key, value) in docs[0].as_hash().unwrap().iter() {
            let key = key.as_str().unwrap().to_string();
            let date = Local
                .datetime_from_str(&(key.clone() + " 00:00:00"), "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .date();
            let name = value.as_str().unwrap();

            let holiday = Holiday::new(name, date);

            holidays.insert(key, holiday);
        }

        Holidays { holidays: holidays }
    }

    pub fn is_holiday(&self, date: Date<Local>) -> bool {
        self.holidays
            .contains_key(&date.format("%Y-%m-%d").to_string())
    }
}
