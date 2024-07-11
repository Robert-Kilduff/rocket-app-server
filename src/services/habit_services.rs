use super::super::DbConn;
use crate::{auth::AuthenticatedUser, models::{HabitUpdate, NewHabit}};
use crate::schema::habits;
use diesel::ExpressionMethods;
use diesel::prelude::*;
use rocket::serde::json::Json;
//use serde_json::json;
use crate::models::Habit;



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


    pub async fn view_habit(&self, user_id: i32, habit_id: i32, auth: &AuthenticatedUser) -> Result<Json<Habit>, HabitUpdateError> {
        if auth.user_id != user_id && auth.role != 1 {
            return Err(HabitUpdateError::AuthorizationError);
        }
    
        let result = self.db.run(move |c| {
            habits::table.filter(habits::id.eq(habit_id).and(habits::user_id.eq(user_id)))
                .get_result::<Habit>(c)
        }).await;
    
        match result {
            Ok(habit) => Ok(Json(habit)),
            Err(diesel::result::Error::NotFound) => Err(HabitUpdateError::NoHabitFound),
            Err(_) => Err(HabitUpdateError::DatabaseError),
        }
    }
    
    //TODO TEST
    pub async fn delete_habit(&self, user_id: i32, habit_id: i32, auth: &AuthenticatedUser) -> Result<(), HabitUpdateError> {
        if auth.role != 1 && auth.user_id != user_id {
            return Err(HabitUpdateError::AuthorizationError);
        }
    
        let result = self.db.run(move |c| {
            diesel::delete(habits::table.filter(habits::id.eq(habit_id).and(habits::user_id.eq(user_id))))
                .execute(c)
        }).await;
    
        match result {
            Ok(count) if count > 0 => Ok(()),
            Ok(_) => Err(HabitUpdateError::NoHabitFound),
            Err(_) => Err(HabitUpdateError::DatabaseError),
        }

    }     

    pub async fn get_habits(&self, auth: &AuthenticatedUser) -> Result<Json<Vec<Habit>>, HabitUpdateError> {
        if auth.role != 1 {
            return Err(HabitUpdateError::AuthorizationError);
        }
        let habits = self.db.run(move |c| {
            habits::table.order(habits::id.desc())
            .limit(1000)
            .order_by(habits::user_id.desc())
            .load::<Habit>(c)
            .map(|habits| rocket::serde::json::Json(habits))
        }).await;

        match habits {
            Ok(habits) => Ok(habits),
            Err(_) => Err(HabitUpdateError::DatabaseError),
        }
    }
}
pub enum HabitUpdateError {
    AuthorizationError,
    DatabaseError,
    NoHabitFound,
}