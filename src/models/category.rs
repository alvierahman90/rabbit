use crate::models::Series;
use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub series: Vec<Series>,
}

impl Category {
    pub fn new(id: i32, new: NewCategory) -> Category {
        Category {
            id,
            name: new.name,
            series: new.series,
        }
    }

    pub fn add_series(&mut self, series: Series) {
        self.series.push(series);
    }
}

pub struct CategoryChangeset {
    pub name: Option<String>,
    pub series: Option<Vec<Series>>,
}

pub struct NewCategory {
    pub name: String,
    pub series: Vec<Series>,
}
