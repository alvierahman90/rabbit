pub mod json;

use crate::models::*;
use std::io;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Generic(String),
    FsIo(String),
    Json(String),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::FsIo(e.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self{
        Self::Json(e.to_string())
    }
}

pub trait Storage {
    fn add_category(&mut self, category: NewCategory) -> Result<i32, Error>;
    fn add_series(&mut self, category_id: i32, series: NewSeries) -> Result<i32, Error>;
    fn add_series_point(&mut self, category_id: i32, series_id: i32, series_point: NewSeriesPoint) -> Result<i32, Error>;
    //fn add_user(&mut self, id: i32, user: NewUser) -> Result<User, Error>;
    fn get_category(&self, id: i32) -> Option<Category>;
    //fn get_series(&self, id: i32) -> Result<Series, Error>;
    //fn get_user(&self, id: i32) -> Result<User, Error>;
    //fn update_category(&mut self, id: i32, changeset: CategoryChangeset) -> Result<(), Error>;
    //fn update_series(&mut self, id: i32, changeset: SeriesChangeset) -> Result<(), Error>;
    //fn update_user(&mut self, id: i32, changeset: UserChangeset) -> Result<(), Error>;
}
