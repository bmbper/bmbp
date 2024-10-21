use chrono::Utc;

pub fn date_time_now() -> String {
    let dt = Utc::now();
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}
