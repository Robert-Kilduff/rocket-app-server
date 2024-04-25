use diesel::{deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use crate::schema::{users, habits};
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
#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
pub struct Habit {
    pub id : i32,
    pub user_id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub max_progress_per_period: Option<i32>,
}
#[derive(Deserialize, Insertable, Default)]
#[diesel(table_name = habits)]
pub struct NewHabit {
    pub user_id: i32,
    pub name: String, 
}