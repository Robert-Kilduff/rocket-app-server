


use diesel::ExpressionMethods;
use diesel::prelude::*;
use rocket::serde::json::{json, Json, Value};

use crate::auth::AuthenticatedUser;
use crate::auth::UserAuth;
use crate::models::{User, NewUser, UserUpdate};
use crate::auth::create_jwt;
use crate::schema::habits::user_id;
use crate::schema::users;
use crate::services::habit_services::HabitUpdateError;
use crate::services::user_services;
use crate::services::user_services::UserService;
use super::super::DbConn;
use rocket::http::Status;
use bcrypt::verify;

//---test login and persist
// curl 127.0.0.1:8000/login -d '{"username": "HELPME", "password": "password"}' 'Content-type: application/json' -X POST 
#[post("/login", data = "<login>")]
pub async fn begin_auth_session(login: Json<UserAuth>, db: DbConn) -> Result<rocket::serde::json::Json<rocket::serde::json::Value>, Status> {
    let username = login.username.clone();
    let password = login.password.clone();
    eprintln!("Attempting to find user: {}", username);
    let user = db.run(move |c| {
        let result = users::table.filter(users::name.eq(username))
        .first::<User>(c).ok();
        eprintln!("Database query result: {:?}", result);
        result
    }).await;

    match user {
        Some(user) => {
            match verify(&password, &user.passhash) {
                Ok(true) => {
                    // Create JWT 
                    match create_jwt(&user.id, user.role) {
                        Ok(token) => Ok(rocket::serde::json::Json(json!({"token": token}))),
                        Err(_) => Err(Status::InternalServerError),
                    }
                },
                Ok(false) => {
                    eprintln!("Password verification failed for user: {}", user.name);
                    Err(Status::Unauthorized)
                },
                Err(e) => {
                    eprintln!("Error verifying password for user: {}: {:?}", user.name, e);
                    Err(Status::Unauthorized)
                },
            }
        },
        None => {
            eprintln!("User not found: {:?}", user);
            Err(Status::Unauthorized)
        },
    }
    
}

#[get("/testJWT")]
pub async fn test_jwt(_auth: AuthenticatedUser) -> Value {
    json!("this statement authenticated")
}
//curl -X GET "http://127.0.0.1:8000/users" -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWJqZWN0Ijo4LCJpYXQiOjE3MTQ0MDI2NDYsImV4cCI6MTcxNDQwMzg0Niwicm9sZSI6MX0.g4a-pqVjcJdOoljPL2RXUb6zbAG97i4ooKoy9YaiLcA"

#[get("/users")]
pub async fn get_users(_auth: AuthenticatedUser, db: DbConn) -> Value {
    let result = match _auth.role {
        1 => db.run(|c| {
            users::table.order(users::id.desc())
                .limit(1000)
                .load::<User>(c)
                .map(|users| json!(users))
                .unwrap_or_else(|_| json!({ "error": "DB ERROR" }))
        }).await,
        _ => {
            println!("Access denied for role: {}", _auth.role);
            json!({ "error": "Access denied" })
        },
    };
    json!(result)
}

#[get("/users/<id>")]
pub async fn view_user(id: i32, _auth: AuthenticatedUser, db: DbConn) -> Value {
    let result = match _auth.role {
        1 => db.run(move |c| {
            users::table.find(id)
            .get_result::<User>(c)
            .expect("DB error selecting user");
        }).await,
        _=> db.run(move |c| {
            users::table.find(_auth.user_id)
            .get_result::<User>(c)
            .expect("DB error selecting user");
        }).await,
    };
    json!(result)
    
}
#[post("/users", format = "json", data = "<new_user>")]
pub async fn create_user_controller(auth: AuthenticatedUser, db: DbConn, mut new_user: Json<NewUser>) -> Result<Json<Value>, (Status, Json<Value>)> {
    let service = UserService::new(db);

    match service.create_user(&auth, new_user).await {
        Ok(_) => Ok(json!({"message": "User created successfully"}).into()),
        Err(e) => match e {
            HabitUpdateError::AuthorizationError => Err((
                Status::Forbidden,
                json!({"error": "Access denied"}).into()
            )),
            HabitUpdateError::DatabaseError => Err((
                Status::InternalServerError,
                json!({"error": "Database error during creation, please try again"}).into()
            )),
            HabitUpdateError::NoHabitFound => Err((
                Status::InternalServerError,
                json!({"error": "Internal error, please try again"}).into()
            )),
        },

    }
    

}

#[put("/users/<id>", format = "json", data = "<user>")]
pub async fn update_users_controller(id: i32, auth: AuthenticatedUser, db: DbConn, user: Json<UserUpdate>) -> Result<Json<Value>, (Status, Json<Value>)> {
    let service = UserService::new(db);
    match service.update_user(id, &auth, &user).await {
        Ok(_) => Ok(Json(json!({"message": "User updated successfully"}))),
        Err(e) => match e {
            HabitUpdateError::AuthorizationError => Err((
                Status::Forbidden,
                Json(json!({"error": "Access denied"}))
            )),
            HabitUpdateError::DatabaseError => Err((
                Status::InternalServerError,
                Json(json!({"error": "Database error during update"}))
            )),
            HabitUpdateError::NoHabitFound => Err((
                Status::NotFound,
                Json(json!({"error": "No user updated"}))
            )),
        },
    }
}


#[delete("/users/<id>")]
pub async fn delete_user_controller(id: i32, auth: AuthenticatedUser, db: DbConn) -> Result<Json<Value>, (Status, Json<Value>)>{
    let service = UserService::new(db);

    match service.delete_user(id, &auth).await {

        Ok(_) => Ok(json!({"message": "user deleted successfully"}).into()),
        Err(e) => match e {
            HabitUpdateError::AuthorizationError => Err((
                Status::Forbidden,
                json!({"error": "Access denied"}).into()
            )),
            HabitUpdateError::DatabaseError => Err((
                Status::InternalServerError,
                json!({"error": "Database error during request"}).into()
            )),
            HabitUpdateError::NoHabitFound => Err((
                Status::NotFound,
                json!({"error": "No user delete"}).into(),
            ))
        }

    }
}