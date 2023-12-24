//pub mod json;
pub mod pg;

use crate::models::*;
use serde_json;
use std::io;

#[derive(Debug)]
pub enum Error {
    Generic(String),
    FsIo(String),
    Json(String),
    PgDb(String),
    Parsing(String),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::FsIo(e.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e.to_string())
    }
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        Self::PgDb(e.to_string())
    }
}

pub trait Storage {
    fn new() -> Self;
    fn add_category(&mut self, category: NewCategory) -> Result<i32, Error>;
    fn add_series(&mut self, series: NewSeries) -> Result<i32, Error>;
    fn add_series_point(&mut self, series_point: NewSeriesPoint) -> Result<i32, Error>;
    fn add_user(&mut self, user: NewUser) -> Result<i32, Error>;
    fn get_categories(&mut self, filter: CategoryFilter) -> Result<Vec<Category>, Error>;
    fn get_series(&mut self, filter: SeriesFilter) -> Result<Vec<Series>, Error>;
    fn get_series_points(&mut self, filter: SeriesPointFilter) -> Result<Vec<SeriesPoint>, Error>;
    fn get_users(&mut self, filter: UserFilter) -> Result<Vec<User>, Error>;
    //fn update_category(&mut self, id: i32, changeset: CategoryChangeset) -> Result<(), Error>;
    //fn update_series(&mut self, id: i32, changeset: SeriesChangeset) -> Result<(), Error>;
    //fn update_user(&mut self, id: i32, changeset: UserChangeset) -> Result<(), Error>;
}
