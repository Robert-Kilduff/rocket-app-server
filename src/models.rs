use diesel::{deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use crate::schema::{users, habits, tasks, task_habit};
#[derive(Serialize, Deserialize, Queryable, AsChangeset, Debug)]
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

#[derive(Deserialize)]
pub struct HabitUpdate {
    pub name: Option<String>,
    pub user_id: Option<i32>,  // Only admin
}

#[derive(Deserialize)]
pub struct UserUpdate {
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub completed_at: Option<NaiveDateTime>,
    pub is_completed: Option<bool>,
    pub complexity: Option<i32>,
}
#[derive(Deserialize)]
pub struct TaskUpdate {
    pub name: Option<String>,
    pub is_completed: Option<bool>,
    pub complexity: Option<i32>,
}
#[derive(Deserialize, Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask {
    pub name: String,
    pub complexity: i32,
    pub is_completed: bool,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct TaskWithHabit {
    #[serde(skip_deserializing)]
    id: i32,
    pub name: String,
    #[serde(skip_deserializing)]
    created_at: chrono::NaiveDateTime,
    completed_at: Option<chrono::NaiveDateTime>,
    is_completed: Option<bool>,
    pub complexity: Option<i32>,
    habit_name: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = task_habit)]
pub struct NewTaskHabit {
    pub task_id: i32,
    pub habit_id: i32,
    pub contribution: Option<i32>,
}   

#[derive(Deserialize)]
pub struct NewTaskRequest {
    pub name: String,
    pub  habit_id: i32,
    pub is_completed: Option<bool>,
    pub complexity: Option<i32>,
    pub contribution: Option<i32>,
}