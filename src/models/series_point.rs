use crate::schema::series_points;
use chrono;
use diesel;
use diesel::prelude::*;
use rocket::form::FromForm;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, PartialEq)]
#[diesel(table_name = series_points)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SeriesPoint {
    pub id: i32,
    pub timestamp: chrono::NaiveDateTime,
    pub value: i32,
    pub series_id: i32,
}

#[derive(FromForm)]
pub struct SeriesPointFilter {
    pub id: Option<i32>,
    pub timestamp_millis: Option<i64>,
    pub value: Option<i32>,
    pub series_id: Option<i32>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = series_points)]
pub struct NewSeriesPoint {
    pub timestamp: chrono::NaiveDateTime,
    pub value: i32,
    pub series_id: i32,
}
