use chrono::prelude::*;

pub fn get_current_naive_date_time() -> NaiveDateTime {
    let now = Utc::now();
    NaiveDate::from_ymd(
        now.year(),
        now.month(),
        now.day()
    ).and_hms(
        now.hour(),
        now.minute(),
        now.second()
    )
}