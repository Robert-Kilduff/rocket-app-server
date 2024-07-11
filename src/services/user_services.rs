use super::{super::DbConn, habit_services::HabitUpdateError};
use crate::{auth::AuthenticatedUser, models::UserUpdate};
use crate::schema::users;
use diesel::{update, ExpressionMethods};
use diesel::prelude::*;
use rocket::serde::json::Json;
//use serde_json::json;
use crate::models::{User, NewUser};

pub struct UserService {
    db: DbConn,
}

impl UserService {
    pub fn new(db: DbConn) -> Self {
        UserService { db }
    } 

    pub async fn update_user(&self, user_id: i32, auth: &AuthenticatedUser, update_data: &Json<UserUpdate>) -> Result<(), HabitUpdateError> {
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

    pub async fn create_user(&self, auth: &AuthenticatedUser, mut new_user: Json<NewUser>) -> Result<(), HabitUpdateError>  {
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
    pub async fn get_users(&self, auth: &AuthenticatedUser) -> Result<Json<Vec<User>>, HabitUpdateError> {
        if auth.role != 1 {
            return Err(HabitUpdateError::AuthorizationError);
        }
        let users = self.db.run(move |c| {
            users::table.order(users::id.desc())
            .limit(1000)
            .load::<User>(c)
            .map(|users| rocket::serde::json::Json(users))
        }).await;
        match users {
            Ok(users) => Ok(users),
            Err(_) => Err(HabitUpdateError::DatabaseError),
        }
    }
    pub async fn view_user(&self, user_id: i32, auth: &AuthenticatedUser) -> Result<Json<User>, HabitUpdateError> {
        if auth.role != 1 && auth.user_id != user_id {
            return Err(HabitUpdateError::AuthorizationError);
        }
        let result = self.db.run(move |c| {
            users::table.filter(users::id.eq(user_id))
                .get_result::<User>(c)
        }).await;

        match result {
            Ok(user) => Ok(Json(user)),
            Err(diesel::result::Error::NotFound) => Err(HabitUpdateError::NoHabitFound),
            Err(_) => Err(HabitUpdateError::DatabaseError),
        }
    }
}