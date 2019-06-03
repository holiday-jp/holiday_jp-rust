use chrono::{Date, Local};

pub struct Holiday {
    pub date: Date<Local>,
    pub name: String,
}

impl Holiday {
    pub fn new(name: String, date: Date<Local>) -> Self {
        Holiday {
            name: name,
            date: date,
        }
    }
}
