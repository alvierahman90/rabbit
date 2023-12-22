use super::series_type::SeriesType;
use chrono;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SeriesPoint {
    pub id: i32,
    pub timestamp: chrono::NaiveDateTime,
    pub value: SeriesType,
}

impl SeriesPoint {
    pub fn new(id: i32, new: NewSeriesPoint) -> SeriesPoint {
        SeriesPoint {
            id,
            timestamp: new.timestamp,
            value: new.value,
        }
    }
}

pub struct NewSeriesPoint {
    pub timestamp: chrono::NaiveDateTime,
    pub value: SeriesType,
}
