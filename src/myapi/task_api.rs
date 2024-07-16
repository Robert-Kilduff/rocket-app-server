use rocket::serde::json::{json, Json, Value};
use rocket::http::Status;


use crate::auth::AuthenticatedUser;
use crate::models::{NewTask, TaskUpdate};
use crate::services::task_services::{TaskService, TaskUpdateError};
use crate::DbConn;

#[get("/tasks")]
pub async fn get_tasks_controller(_auth: AuthenticatedUser, db: DbConn) -> Result<Json<Value>, (Status, Json<Value>)> {
    let service = TaskService::new(db);
    match service.get_tasks(&_auth).await {
        Ok(tasks) => Ok(json!({
            "result": *tasks,
            "message": "Tasks returned successfully"})
        .into()),
        Err(e) => match e {
            TaskUpdateError::AuthorizationError => Err((
                Status::Forbidden,
                json!({"error": "Access denied"}).into()
            )),
            TaskUpdateError::DatabaseError => Err((
                Status::InternalServerError,
                json!({"error": "Database error"}).into()
            )),
            TaskUpdateError::NoTaskFound => Err((
                Status::NoContent,
                json!({"error": "No tasks found"}).into()
            )),
        },
    }
}

#[get("/users/<task_user_id>/tasks/<task_id>")]
pub async fn view_task_controller(task_user_id: i32, task_id: i32, auth: AuthenticatedUser, db: DbConn) -> Result<Json<Value>, (Status, Json<Value>)> {
    let service = TaskService::new(db);

    match service.view_task(task_user_id, task_id, &auth).await {
        Ok(task) => Ok(json!({
            "result": *task,
            "message": "Task returned successfully"
        }).into()),
        Err(e) => match e {
            TaskUpdateError::AuthorizationError => Err((
                Status::Forbidden,
                json!({"error": "Access denied"}).into()
            )),
            TaskUpdateError::DatabaseError => Err((
                Status::InternalServerError,
                json!({"error": "Database error during search, is user id correct?"}).into()
            )),
            TaskUpdateError::NoTaskFound => Err((
                Status::NoContent,
                json!({"error": "No task found"}).into()
            )),
        },
    }
}