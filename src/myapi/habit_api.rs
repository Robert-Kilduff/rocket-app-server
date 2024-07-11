use rocket::serde::json::{json, Json, Value};
use rocket::http::Status;


use crate::auth::AuthenticatedUser;
use crate::models::{NewHabit, HabitUpdate};
use crate::services::habit_services::{HabitService, HabitUpdateError};
use crate::DbConn;






#[get("/habits")]
pub async fn get_habits_controller(_auth: AuthenticatedUser, db: DbConn) -> Result<Json<Value>, (Status, Json<Value>)> {
    let service = HabitService::new(db);
    match service.get_habits(&_auth).await {
        Ok(habits) => Ok(json!({
            "result": *habits,
            "message": "Habits returned successfully"})
        .into()),
        Err(e) => match e {
            HabitUpdateError::AuthorizationError => Err((
                Status::Forbidden,
                json!({"error": "Access denied"}).into()
            )),
            HabitUpdateError::DatabaseError => Err((
                Status::InternalServerError,
                json!({"error": "Database error"}).into()
            )),
            HabitUpdateError::NoHabitFound => Err((
                Status::NoContent,
                json!({"error": "No habits found"}).into()
            )),
        },
    }
}



//TODO security in architecture here?
#[get("/users/<habit_user_id>/habits/<habit_id>")]
pub async fn view_habit_controller(habit_user_id: i32, habit_id: i32, auth: AuthenticatedUser, db: DbConn) -> Result<Json<Value>, (Status, Json<Value>)> {
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
pub async fn create_habit_controller(habit_user_id: i32, auth: AuthenticatedUser, db: DbConn, new_habit: Json<NewHabit>) -> Result<Json<Value>, (Status, Json<Value>)> {
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
pub async fn update_habit_controller(habit_user_id: i32, habit_id: i32, auth: AuthenticatedUser, db: DbConn, habit: Json<HabitUpdate>) -> Result<Json<Value>, (Status, Json<Value>)> {
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
pub async fn delete_habit_controller(habit_user_id: i32, habit_id: i32, auth: AuthenticatedUser, db: DbConn) -> Result<Json<Value>, (Status, Json<Value>)> {
    let service = HabitService::new(db);
    match service.delete_habit(habit_user_id, habit_id, &auth).await {
        Ok(_) => Ok(json!({"message": "habit deleted", "id": habit_id}).into()),
        Err(e) => match e {
            HabitUpdateError::AuthorizationError => Err((
                Status::Forbidden,
                json!({"error": "Access denied"}).into(),
            )),
            HabitUpdateError::DatabaseError => Err((
                Status::InternalServerError,
                json!({"error": "DB error deleting habit"}).into(),
            )),
            HabitUpdateError::NoHabitFound => Err((
                Status::NotFound,
                json!({"error": "No habit found"}).into(),
            )),
        },
    }
}
