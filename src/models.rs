
use chrono::prelude::*;
use diesel::deserialize::Queryable;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

