use std::collections::HashMap;
use std::convert::TryFrom;
use time::Date;

lazy_static! {
    static ref EN_HOLIDAY_NAMES: HashMap<&'static str, &'static str> = {
        let holiday_name_pairs = [
            ["元日", "New Year's Day"],
            ["成人の日", "Coming of Age Day"],
            ["建国記念の日", "National Foundation Day"],
            ["春分の日", "Vernal Equinox Day"],
            ["憲法記念日", "Constitution Memorial Day"],
            ["みどりの日", "Greenery Day"],
            ["こどもの日", "Children's Day"],
            ["海の日", "Marine Day"],
            ["山の日", "Mountain Day"],
            ["敬老の日", "Respect for the Aged Day"],
            ["秋分の日", "Autumnal Equinox Day"],
            ["体育の日", "Health and Sports Day"],
            ["スポーツの日", "Sports Day"],
            ["文化の日", "National Culture Day"],
            ["勤労感謝の日", "Labor Thanksgiving Day"],
            ["天皇誕生日", "Emperor's Birthday"],
            ["昭和の日", "Showa Day"],
            ["振替休日", "Holiday in lieu"],
            ["国民の休日", "Citizen's Holiday"],
            [
                "即位礼正殿の儀",
                "The Ceremony of the Enthronement of His Majesty th Emperor (at the Seiden)",
            ],
            [
                "昭和天皇の大喪の礼",
                "The Funeral Ceremony of Emperor Showa.",
            ],
            [
                "皇太子徳仁親王の結婚の儀",
                "The Rite of Wedding of HIH Crown Prince Naruhito",
            ],
        ];
        let mut names = HashMap::with_capacity(22);

        for pair in holiday_name_pairs.iter() {
            names.insert(pair[0], pair[1]);
        }

        names
    };
}

pub struct Holiday {
    pub date: Date,
    pub name: String,
}

impl Holiday {
    const WDAY_NAMES: [&'static str; 7] = ["日", "月", "火", "水", "木", "金", "土"];

    pub fn new(name: &str, date: Date) -> Self {
        Holiday {
            name: name.to_string(),
            date: date,
        }
    }

    pub fn name_en(&self) -> String {
        EN_HOLIDAY_NAMES.get::<str>(&self.name).unwrap().to_string()
    }

    pub fn wday_name(&self) -> String {
        let wday = self.date.weekday().number_days_from_sunday();
        let wday_usize = usize::try_from(wday).unwrap();

        Self::WDAY_NAMES[wday_usize].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Holiday;
    use time::{Date, Month};

    #[test]
    fn name_en() {
        let date = Date::from_calendar_date(1970, Month::January, 1).unwrap();
        let holiday = Holiday::new("元日", date);

        assert_eq!(holiday.name_en(), "New Year's Day");
    }

    #[test]
    fn wday_name() {
        let date = Date::from_calendar_date(1970, Month::January, 1).unwrap();
        let holiday = Holiday::new("元日", date);

        assert_eq!(holiday.wday_name(), "木");
    }
}
