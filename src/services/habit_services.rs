use super::super::DbConn;
use crate::{auth::AuthenticatedUser, models::{HabitUpdate, NewHabit}};
use crate::schema::habits;
use diesel::ExpressionMethods;
use diesel::prelude::*;
use rocket::serde::json::{json, Json};


pub struct HabitService {
    db: DbConn,
}

impl HabitService {
    pub fn new(db: DbConn) -> Self {
        HabitService { db }
    }

    pub async fn update_habit(&self, user_id: i32, habit_id: i32, auth: &AuthenticatedUser, update_data: &Json<HabitUpdate>) -> Result<(), HabitUpdateError> {
        if auth.role != 1 && auth.user_id != user_id {
            return Err(HabitUpdateError::AuthorizationError);
        }

        let name = update_data.name.to_owned().expect("Name Failure");
        let result = self.db.run(move |c| {
            diesel::update(habits::table.filter(habits::id.eq(habit_id).and(habits::user_id.eq(user_id))))
                .set(habits::name.eq(name))
                .execute(c)
        }).await;

        match result {
            Ok(count) if count > 0 => Ok(()),
            Ok(_) => Err(HabitUpdateError::NoHabitFound),
            Err(_) => Err(HabitUpdateError::DatabaseError),
        }
    }

    pub async fn create_habit(&self, user_id: i32, auth: &AuthenticatedUser, new_data: Json<NewHabit>) -> Result<(), HabitUpdateError> {
        if auth.role != 1 && auth.user_id != user_id {
            return Err(HabitUpdateError::AuthorizationError);
        }
        let result = self.db.run(move |c| {
            diesel::insert_into(habits::table)
            .values(&*new_data)
            .execute(c)
        }).await;

        match result {
            Ok(count) if count > 0 => Ok(()),
            Ok(_) => Err(HabitUpdateError::NoHabitFound),
            Err(_) => Err(HabitUpdateError::DatabaseError),
        }
    }
}

pub enum HabitUpdateError {
    AuthorizationError,
    DatabaseError,
    NoHabitFound,
}