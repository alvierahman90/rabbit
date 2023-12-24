use crate::schema::users;
use diesel;
use diesel::prelude::*;
use rocket::form::FromForm;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, PartialEq)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(FromForm)]
pub struct UserFilter {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct UserChangeset {
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}
