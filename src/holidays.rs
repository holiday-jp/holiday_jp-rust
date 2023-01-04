use std::collections::HashMap;

use time::{Date, macros::format_description};
use yaml_rust::YamlLoader;

use super::holiday::Holiday;

pub struct Holidays {
    pub holidays: HashMap<String, Holiday>,
}

impl Holidays {
    pub fn new() -> Self {
        const HOLIDAYS_STRING: &str = include_str!("../holiday_jp/holidays.yml");
        let docs = YamlLoader::load_from_str(HOLIDAYS_STRING).unwrap();
        let mut holidays = HashMap::new();

        for (key, value) in docs[0].as_hash().unwrap().iter() {
            let key = key.as_str().unwrap().to_string();
            let date = Date::parse(&key, format_description!("[year]-[month]-[day]")).unwrap();
            let name = value.as_str().unwrap();
            let holiday = Holiday::new(name, date);

            holidays.insert(key, holiday);
        }

        Holidays { holidays }
    }

    pub fn is_holiday(&self, date: Date) -> bool {
        self.holidays.contains_key(
            &date
                .format(&format_description!("[year]-[month]-[day]"))
                .unwrap(),
        )
    }
}

impl Default for Holidays {
    fn default() -> Self {
        Self::new()
    }
}