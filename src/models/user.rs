use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

pub struct UserChangeset {
    pub name: Option<String>,
    pub email: Option<String>,
}

pub struct NewUser {
    pub name: String,
    pub email: String,
}
