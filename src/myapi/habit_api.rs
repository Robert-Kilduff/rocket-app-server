use diesel::ExpressionMethods;
use diesel::prelude::*;
use rocket::serde::json::{json, Json, Value};
use rocket::response::status;
//use serde_json::json;
use crate::auth::AuthenticatedUser;
use crate::models::HabitUpdate;
use crate::models::{User, Habit, NewHabit};
use crate::auth::BasicAuth;
use crate::schema::habits;
use crate::schema::habits::user_id;
use super::super::DbConn;
use crate::services::habit_services::{HabitService, HabitUpdateError};


#[get("/habits")]
pub async fn get_habits(_auth: AuthenticatedUser, db: DbConn) -> Value {
    let query_result = db.run(|c| {
        habits::table
            .order(habits::id.desc())
            .limit(1000)
            .load::<Habit>(c)
    }).await;

    match query_result {
        Ok(habits) => {
            if habits.is_empty() {
                json!({"error": "No habits found"})
            } else {
                json!(habits)
            }
        },
        Err(e) => {
            eprintln!("Failed to fetch habits: {}", e);
            json!({"error": "Database error"})
        }
    }
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
//handle habit_id input properlyy
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
//review this
#[put("/users/<habit_user_id>/habits/<habit_id>", format = "json", data = "<habit>")]
pub async fn update_habit(habit_user_id: i32, habit_id: i32, _auth: AuthenticatedUser, db: DbConn, habit: Json<Habit>) -> Value {

    let result = if _auth.role == 1 {  
        db.run(move |c| {
            diesel::update(habits::table.filter(habits::id.eq(habit_id)))
                .set(habits::name.eq(habit.name.to_owned()))
                .execute(c)
        }).await
    } else {  
        db.run(move |c| {
            diesel::update(habits::table.filter(habits::id.eq(habit_id).and(habits::user_id.eq(habit_user_id))))
                .set(habits::name.eq(habit.name.to_owned()))
                .execute(c)
        }).await
    }; // some dup code here but diesel is not happy with anything cleaner.

    // Handle the result
    match result {
        Ok(count) if count > 0 => json!({"message": "Habit updated successfully", "id": habit_id}),
        Ok(_) => json!({"error": "No habit updated"}), //no habit, not owned
        Err(_) => json!({"error": "Database error during update"}),
    }
}

//-- fix of above...test & repeat.
#[put("/users/test/<habit_user_id>/habits/<habit_id>", format = "json", data = "<habit>")]
pub async fn update_habit_controller(habit_user_id: i32, habit_id: i32, _auth: AuthenticatedUser, db: DbConn, habit: Json<HabitUpdate>) -> Value {
    let service = HabitService::new(db);
    match service.update_habit(habit_user_id, habit_id, &_auth, &habit).await {
        Ok(_) => json!({"message": "Habit updated successfully"}),
        Err(e) => match e {
            HabitUpdateError::AuthorizationError => json!({"error": "Access denied"}),
            HabitUpdateError::DatabaseError => json!({"error": "Database error during update"}),
            HabitUpdateError::NoHabitFound => json!({"error": "No habit updated"}),
        },
    }
}



//---

#[delete("/users/<habit_user_id>/habits/<habit_id>")]
pub async fn delete_habit(habit_user_id: i32, habit_id: i32, _auth: AuthenticatedUser, db: DbConn) -> Value {
    if _auth.user_id != habit_user_id && _auth.role != 1 {
        return json!({"error": "Access denied"});
    }
    let result = db.run(move |c| {
        diesel::delete(habits::table.find(habit_id))
            .execute(c)
    }).await;

    match result {
        Ok(_) => json!({"message": "habit deleted", "id": habit_id}),
        Err(_) => json!({"error": "DB error deleting habit"}),
    }
}
