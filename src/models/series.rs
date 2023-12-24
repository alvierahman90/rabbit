use crate::schema::series;
use diesel;
use diesel::prelude::*;
use rocket::form::FromForm;
use serde::{Deserialize, Serialize};
use serde_with;

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, PartialEq)]
#[diesel(table_name = series)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Series {
    pub id: i32,
    pub name: String,
    pub repeat: i32,
    pub good: bool,
    pub category_id: i32,
}

#[derive(FromForm)]
pub struct SeriesFilter {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub repeat: Option<i32>,
    pub good: Option<bool>,
    pub category_id: Option<i32>,
}

#[derive(AsChangeset)]
#[diesel(table_name = series)]
pub struct SeriesChangeset {
    pub name: Option<String>,
    pub repeat: Option<i32>,
    pub good: Option<bool>,
    pub category_id: Option<i32>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = series)]
pub struct NewSeries {
    pub name: String,
    pub repeat: i32,
    pub good: bool,
    pub category_id: i32,
}
