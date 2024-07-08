use super::{super::DbConn, habit_services::HabitUpdateError};
use crate::{auth::AuthenticatedUser, models::UserUpdate};
use crate::schema::users;
use diesel::{update, ExpressionMethods};
use diesel::prelude::*;
use rocket::serde::json::{self, json, Json as JsonValue};
//use serde_json::json;
use crate::models::{Habit, HabitUpdate, NewUser};

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

    pub async fn create_user(&self, auth: &AuthenticatedUser, mut new_user: JsonValue<NewUser>) -> Result<(), HabitUpdateError>  {
        if auth.role != 1 {
            return Err(HabitUpdateError::AuthorizationError);
        }

        new_user.hashgen();

        let result = self.db.run(move |c| {
            diesel::insert_into(users::table)
            .values(&*new_user)
            .execute(c)
        }).await;

        match result {
            Ok(count) if count > 0 => Ok(()),
            Ok(_) => Err(HabitUpdateError::NoHabitFound),
            Err(_) => Err(HabitUpdateError::DatabaseError),
        }


    }

    pub async fn delete_user(&self, user_id: i32, auth: &AuthenticatedUser) -> Result<(), HabitUpdateError> {
        if auth.role != 1 && auth.user_id != user_id {
            return Err(HabitUpdateError::AuthorizationError);
        }

        let result = self.db.run(move |c| {
            diesel::delete(users::table.filter(users::id.eq(user_id)))
            .execute(c)
        }).await;

        match result {
            Ok(count) if count > 0  => Ok(()),
            Ok(_) => Err(HabitUpdateError::NoHabitFound),
            Err(_) => Err(HabitUpdateError::DatabaseError),
        
        }
    }
}