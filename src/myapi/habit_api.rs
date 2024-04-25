use diesel::ExpressionMethods;
use diesel::prelude::*;
use rocket::serde::json::{json, Json, Value};
use rocket::response::status;
//use serde_json::json;
use crate::auth::AuthenticatedUser;
use crate::models::{User, Habit, NewHabit};
use crate::auth::BasicAuth;
use crate::schema::habits;
use crate::schema::habits::user_id;
use super::super::DbConn;

#[get("/habits")]
pub async fn get_habits(_auth: AuthenticatedUser, db: DbConn) -> Value {
    let result = match _auth.role {
        1 => {
            db.run(|c| {
                habits::table.order(habits::id.desc())
                .limit(1000).load::<Habit>(c)
                .expect("DB ERROR");
             }).await
        },
        _=> {
            db.run(|c| {
                habits::table.order(habits::id.desc())
                .limit(1000).load::<Habit>(c)
                .expect("DB ERROR");
             }).await
        }
    };
    json!(result)
    
}

//TODO security in architecture here?
#[get("/users/<habit_user_id>/habits/<habit_id>")]
pub async fn view_habit(habit_user_id: i32, habit_id: i32, _auth: AuthenticatedUser, db: DbConn) -> Value {
    if _auth.user_id != habit_user_id && _auth.role != 1 {
        return json!({"error": "Access denied"});
    }
    let result = match _auth.role {
        1 => db.run(move |c| {
            habits::table.filter(habits::id.eq(habit_id).and(habits::user_id.eq(&user_id)))
            .get_result::<Habit>(c)
            .expect("DB error selecting user");
        }).await,
        
        _=> db.run(move |c| {
            habits::table.filter(habits::id.eq(habit_id).and(habits::user_id.eq(&_auth.user_id)))
            .get_result::<Habit>(c)
            .expect("DB error selecting user");
        }).await,
    };
    json!(result)
    
}

#[post("/users/<habit_user_id>/habits", format = "json", data = "<new_habit>")]
pub async fn create_habit(habit_user_id: i32, _auth: AuthenticatedUser, db: DbConn, mut new_habit: Json<NewHabit>) -> Value {
    match _auth.role {
        1 => {
            new_habit.user_id = habit_user_id;
            db.run(move |c| {
                let result = diesel::insert_into(habits::table)
                .values(&*new_habit)
                .execute(c)
                .expect("DB ERROR INSERTING");
            json!(result)
            }).await
        },
        _=> json!({"error": "Access Denied"})
    }
    

}

#[put("/users/<habit_user_id>/habits/<habit_id>", format = "json", data = "<habit>")]
pub async fn update_habits(habit_user_id: i32,habit_id: i32, _auth: AuthenticatedUser, db: DbConn, habit: Json<Habit>) -> Value {
    let result = match _auth.role {
        1 => {
            db.run(move |c| {
                diesel::update(habits::table.find(habit_id)) //more efficient to filter by users first?
                .set((
                    habits::name.eq(habit.name.to_owned()),
                    habits::user_id.eq(habit.user_id.to_owned())
                    //set defaults
                ))
                .execute(c)
                .expect("DB error updating habit");
            }).await
        },
        _ => {
            db.run(move |c| {
                diesel::update(habits::table.filter(habits::user_id.eq(habit_user_id)).find(habit_id))
                .set(
                    habits::name.eq(habit.name.to_owned())
                )
                .execute(c)
                .expect("DB error updating habit");
                unimplemented!()

            }).await
        },
    };   
    json!(result)
}

