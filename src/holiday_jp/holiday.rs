use chrono::{Date, Local};

pub struct Holiday {
    pub date: Date<Local>,
    pub name: String,
}

impl Holiday {
    pub fn new(name: &str, date: Date<Local>) -> Self {
        Holiday {
            name: name.to_string(),
            date: date,
        }
    }
}
