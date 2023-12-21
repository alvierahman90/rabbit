use chrono;
use super::series_type::SeriesType;

#[derive(Debug)]
pub struct SeriesPoint {
    id: i32,
    timestamp: chrono::NaiveDateTime,
    value: SeriesType,
}

impl SeriesPoint {
    pub fn new(id: i32, timestamp: chrono::NaiveDateTime, value: SeriesType) -> SeriesPoint {
        SeriesPoint {
            id,
            timestamp,
            value
        }
    }
}
