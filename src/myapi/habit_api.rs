use diesel::ExpressionMethods;
use diesel::prelude::*;
use rocket::serde::json::{json, Json as JsonValue, Value};
use rocket::http::Status;
use serde::{Serialize, Deserialize};

use crate::auth::AuthenticatedUser;
use crate::models::{Habit, NewHabit, HabitUpdate};
use crate::schema::habits;
use crate::services::habit_services::{HabitService, HabitUpdateError};
use crate::DbConn;






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
pub async fn view_habit_controller(habit_user_id: i32, habit_id: i32, auth: AuthenticatedUser, db: DbConn) -> Result<JsonValue<Value>, (Status, JsonValue<Value>)> {
    let service = HabitService::new(db);

    match service.view_habit(habit_user_id, habit_id, &auth).await {
        Ok(habit) => Ok(json!({
            "result": *habit,
            "message": "Habit returned successfully"
        }).into()),
        Err(e) => match e {
            HabitUpdateError::AuthorizationError => Err((
                Status::Forbidden,
                json!({"error": "Access denied"}).into()
            )),
            HabitUpdateError::DatabaseError => Err((
                Status::InternalServerError,
                json!({"error": "Database error during search, is user id correct?"}).into()
            )),
            HabitUpdateError::NoHabitFound => Err((
                Status::NoContent,
                json!({"error": "No habit found"}).into()
            )),
        },
    }
}



//server side should not return 200 OK on fail, TODO. Json rocket custom err.
#[post("/users/<habit_user_id>/habits", format = "json", data = "<new_habit>")]
pub async fn create_habit_controller(habit_user_id: i32, auth: AuthenticatedUser, db: DbConn, new_habit: JsonValue<NewHabit>) -> Result<JsonValue<Value>, (Status, JsonValue<Value>)> {
    let service = HabitService::new(db);

    match service.create_habit(habit_user_id, &auth, new_habit).await {
        Ok(_) => Ok(json!({"message": "Habit created successfully"}).into()),
        Err(e) => match e {
            HabitUpdateError::AuthorizationError => Err((
                Status::Forbidden,
                json!({"error": "Access denied"}).into()
            )),
            HabitUpdateError::DatabaseError => Err((
                Status::InternalServerError,
                json!({"error": "Database error during creation, is user id correct?"}).into()
            )),
            HabitUpdateError::NoHabitFound => Err((
                Status::NoContent,
                json!({"error": "No habit updated"}).into()
            )),
        },
    }
    

}

#[put("/users/<habit_user_id>/habits/<habit_id>", format = "json", data = "<habit>")]
pub async fn update_habit_controller(habit_user_id: i32, habit_id: i32, auth: AuthenticatedUser, db: DbConn, habit: JsonValue<HabitUpdate>) -> Result<JsonValue<Value>, (Status, JsonValue<Value>)> {
    let service = HabitService::new(db);
    match service.update_habit(habit_user_id, habit_id, &auth, &habit).await {
        Ok(_) => Ok(json!({"message": "Habit updated successfully"}).into()),
        Err(e) => match e {
            HabitUpdateError::AuthorizationError => Err((
                Status::Forbidden,
                json!({"error": "Access denied"}).into()
            )),
            HabitUpdateError::DatabaseError => Err((
                Status::InternalServerError,
                json!({"error": "Database error during update"}).into()
            )),
            HabitUpdateError::NoHabitFound => Err((
                Status::NotFound,
                json!({"error": "No habit updated"}).into(),
            ))
        },
    }
}



#[delete("/users/<habit_user_id>/habits/<habit_id>")]
pub async fn delete_habit(habit_user_id: i32, habit_id: i32, auth: AuthenticatedUser, db: DbConn) -> Value {
    if auth.user_id != habit_user_id && auth.role != 1 {
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
