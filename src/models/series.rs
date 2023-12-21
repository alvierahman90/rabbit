use chrono;
use super::series_point::SeriesPoint;
use serde::{ Serialize, Deserialize };
use serde_with;

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Series {
    pub id: i32,
    pub name: String,
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    pub repeat: chrono::Duration,
    pub good: bool,
    pub points: Vec<SeriesPoint>,
}

impl Series {
    pub fn new(id: i32, series: NewSeries) -> Series {
        Series {
            id,
            name: series.name,
            repeat: series.repeat,
            good: series.good,
            points: series.points,
        }
    }

    pub fn add_point(&mut self, point: SeriesPoint) {
        self.points.push(point);
    }
}

pub struct SeriesChangeset {
    pub name: Option<String>,
    pub repeat: Option<chrono::Duration>,
    pub good: Option<bool>,
    pub points: Option<Vec<SeriesPoint>>,
}

pub struct NewSeries {
    pub name: String,
    pub repeat: chrono::Duration,
    pub good: bool,
    pub points: Vec<SeriesPoint>,
}
