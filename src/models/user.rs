use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: i32,
    name: String,
    email: String,
}

pub struct UserChangeset {
    name: Option<String>,
    email: Option<String>,
}

pub struct NewUser {
    name: String,
    email: String,
}
