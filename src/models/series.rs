use chrono;
use super::series_point::SeriesPoint;

#[derive(Debug)]
pub struct Series {
    id: i32,
    name: String,
    repeat: chrono::Duration,
    good: bool,
    points: Vec<SeriesPoint>,
}

impl Series {
    pub fn new(id: i32, name: String, repeat: chrono::Duration, good: bool) -> Series {
        Series {
            id,
            name,
            repeat,
            good,
            points: vec![],
        }
    }

    pub fn add_point(&mut self, point: SeriesPoint) {
        self.points.push(point);
    }
}
