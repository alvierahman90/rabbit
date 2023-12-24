use crate::schema::categories;
use diesel;
use diesel::prelude::*;
use rocket::form::FromForm;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, PartialEq)]
#[diesel(table_name = categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
}

impl Category {}

#[derive(FromForm)]
pub struct CategoryFilter {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub user_id: Option<i32>,
}

pub struct CategoryChangeset {
    pub name: Option<String>,
}

#[derive(Insertable, Deserialize, FromForm)]
#[diesel(table_name = categories)]
pub struct NewCategory {
    pub name: String,
    pub user_id: i32,
}
