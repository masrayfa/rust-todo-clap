use chrono::NaiveDateTime;

// parse the date string into a NaiveDate
pub fn parse_date(date: &str) -> NaiveDateTime {
    let date: NaiveDateTime = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S").unwrap();
    date
}