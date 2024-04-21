use diesel::{deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use crate::schema::users;
#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub passhash: String,
    #[serde(skip_deserializing)]
    pub role: i32,
    #[serde(skip_deserializing)]
    created_at: NaiveDateTime,
}
#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String, 
    pub email: String,
    pub role: Option<i32>,
    pub passhash: String,
}