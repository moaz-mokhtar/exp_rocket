// use chrono::NaiveDate;
use super::schema::{users};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Queryable, Debug, Serialize, Deserialize, Clone,FromForm)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password :String
}

#[derive(Insertable,Debug,FromForm,Clone)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password :String
}
