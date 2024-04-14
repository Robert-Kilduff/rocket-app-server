use diesel::deserialize::Queryable;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Queryable)]
pub struct User {
    id: i32,
    name: String,
    email: String,
    role: Option<i32>,
    created_at: NaiveDateTime,
}