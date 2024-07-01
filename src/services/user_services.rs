use super::{super::DbConn, habit_services::HabitUpdateError};
use crate::{auth::AuthenticatedUser, models::UserUpdate};
use crate::schema::users;
use diesel::{update, ExpressionMethods};
use diesel::prelude::*;
use rocket::serde::json::{json, Json as JsonValue};
//use serde_json::json;
use crate::models::Habit;

pub struct UserService {
    db: DbConn,
}

impl UserService {
    pub fn new(db: DbConn) -> Self {
        UserService { db }
    } 

    pub async fn update_user(&self, user_id: i32, auth: &AuthenticatedUser, update_data: &JsonValue<UserUpdate>) -> Result<(), HabitUpdateError> {
        if auth.role != 1 && auth.user_id != user_id {
            return Err(HabitUpdateError::AuthorizationError);
        }

        let name = update_data.name.to_owned();
        let email = update_data.email.to_owned();

        let result = self.db.run(move |c| {
            match (email, name) {
                (Some(email), Some(name)) => {
                    update(users::table.filter(users::id.eq(user_id)))
                        .set((users::email.eq(email), users::name.eq(name)))
                        .execute(c)
                },
                (Some(email), None) => {
                    update(users::table.filter(users::id.eq(user_id)))
                        .set(users::email.eq(email))
                        .execute(c)
                },
                (None, Some(name)) => {
                    update(users::table.filter(users::id.eq(user_id)))
                        .set(users::name.eq(name))
                        .execute(c)
                },
                (None, None) => {
                    return Err(diesel::result::Error::NotFound);
                },
            }
        }).await;

        match result {
            Ok(count) if count > 0 => Ok(()),
            Ok(_) => Err(HabitUpdateError::NoHabitFound),
            Err(_) => Err(HabitUpdateError::DatabaseError),
        }
    }
}