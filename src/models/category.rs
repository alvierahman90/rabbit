use crate::models::Series;

#[derive(Debug)]
pub struct Category {
    id: i32,
    name: String,
    series: Vec<Series>,
}

impl Category {
    pub fn new(id: i32, name: String) -> Category {
        Category {
            id,
            name,
            series: vec![],
        }
    }

    pub fn add_series(&mut self, series: Series) {
        self.series.push(series);
    }
}
