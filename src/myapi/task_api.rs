use rocket::serde::json::{json, Json, Value};
use rocket::http::Status;


use crate::auth::AuthenticatedUser;
use crate::models::NewTaskRequest;
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

#[post("/users/<_task_user_id>/tasks", data = "<new_task>")]
pub async fn create_task_controller(_task_user_id: i32, new_task: Json<NewTaskRequest>, auth: AuthenticatedUser, db: DbConn) -> Result<Json<Value>, (Status, Json<Value>)> {
    let service = TaskService::new(db);

    match service.create_task(new_task, &auth).await {
        Ok(task) => Ok(json!({
            "result": task,
            "message": "Task created successfully"
        }).into()),
        Err(e) => match e {
            TaskUpdateError::AuthorizationError => Err((
                Status::Forbidden,
                json!({"error": "Access denied"}).into()
            )),
            TaskUpdateError::DatabaseError => Err((
                Status::InternalServerError,
                json!({"error": "Database error during task creation"}).into()
            )),
            TaskUpdateError::NoTaskFound => Err((
                Status::NoContent,
                json!({"error": "No task found"}).into()
            )),
            
        },
    }
}

#[put("/users/<_task_user_id>/tasks/<task_id>", data = "<task>")]
pub async fn update_task_controller(_task_user_id: i32, task_id: i32, task: Json<NewTaskRequest>, auth: AuthenticatedUser, db: DbConn) -> Result<Json<Value>, (Status, Json<Value>)> {
    let service = TaskService::new(db);

    match service.update_task(task_id, task, &auth).await {
        Ok(task) => Ok(json!({
            "result": task,
            "message": "Task updated successfully"
        }).into()),
        Err(e) => match e {
            TaskUpdateError::AuthorizationError => Err((
                Status::Forbidden,
                json!({"error": "Access denied"}).into()
            )),
            TaskUpdateError::DatabaseError => Err((
                Status::InternalServerError,
                json!({"error": "Database error during task update"}).into()
            )),
            TaskUpdateError::NoTaskFound => Err((
                Status::NoContent,
                json!({"error": "No task found"}).into()
            )),
        },
    }
}

#[delete("/users/<_task_user_id>/tasks/<task_id>")]
pub async fn delete_task_controller(_task_user_id: i32, task_id: i32, auth: AuthenticatedUser, db: DbConn) -> Result<Json<Value>, (Status, Json<Value>)> {
    let service = TaskService::new(db);

    match service.delete_task(task_id, &auth).await {
        Ok(_) => Ok(json!({
            "message": "Task deleted successfully"
        }).into()),
        Err(e) => match e {
            TaskUpdateError::AuthorizationError => Err((
                Status::Forbidden,
                json!({"error": "Access denied"}).into()
            )),
            TaskUpdateError::DatabaseError => Err((
                Status::InternalServerError,
                json!({"error": "Database error during task deletion"}).into()
            )),
            TaskUpdateError::NoTaskFound => Err((
                Status::NoContent,
                json!({"error": "No task found"}).into()
            )),
        },
    }
}