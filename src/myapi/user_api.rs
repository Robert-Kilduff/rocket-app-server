

use bcrypt::hash;
use diesel::ExpressionMethods;
use diesel::prelude::*;
use rocket::serde::json::{json, Json, Value};
use rocket::response::status;
use crate::auth::AuthenticatedUser;
use crate::auth::UserAuth;
use crate::models::{User, NewUser};
use crate::auth::{BasicAuth, create_jwt};
use crate::schema::users;
use super::super::DbConn;
use rocket::http::Status;
use bcrypt::{verify, DEFAULT_COST};

//---test login and persist
// curl 127.0.0.1:8000/login -d '{"username": "HELPME", "password": "password"}' 'Content-type: application/json' -X POST 
#[post("/login", data = "<login>")]
pub async fn begin_auth_session(login: Json<UserAuth>, db: DbConn) -> Result<rocket::serde::json::Json<rocket::serde::json::Value>, Status> {
    let username = login.username.clone();
    let password = login.password.clone();
    let user = db.run(move |c| {
        users::table.filter(users::name.eq(username))
        .first::<User>(c).ok()
    }).await;

    match user {
        Some(user) if verify(&password, &user.passhash).unwrap_or(false) => { 
            // Create JWT 
            match create_jwt(&user.id, user.role) {
                Ok(token) => Ok(rocket::serde::json::Json(json!({"token": token}))),
                Err(_) => Err(Status::InternalServerError),
            }
        },
        _ => Err(Status::Unauthorized),
    }
}

#[get("/testJWT")]
pub async fn test_jwt(_auth: AuthenticatedUser) -> Value {
    json!("this statement authenticated")
}
//curl -X GET "http://127.0.0.1:8000/testJWT" -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWJqZWN0Ijo0LCJpYXQiOjE3MTM3ODE2ODUsImV4cCI6MTcxMzc4Mjg4NSwicm9sZSI6Mn0.Jz1m-QDF6AfjXm3lw5Ci36-sf8o4vvA4WOnIQva248w"

//---test
#[get("/users")]
pub async fn get_users(_auth: BasicAuth, db: DbConn) -> Value {
    db.run(|c| {
       let users =  users::table.order(users::id.desc())
       .limit(1000).load::<User>(c)
       .expect("DB ERROR");
       json!(users)
    }).await
    
}

#[get("/users/<id>")]
pub async fn view_user(id: i32, _auth: BasicAuth , db: DbConn) -> Value {
    db.run(move |c| {
        let user = users::table.find(id)
        .get_result::<User>(c)
        .expect("DB error selecting user");
        json!(user)
    }).await
}

#[post("/users", format = "json", data = "<new_user>")]
pub async fn create_user(_auth: BasicAuth, db: DbConn, mut new_user: Json<NewUser>) -> Value {
    new_user.hashgen(); //more explicitly .into_inner() not mut, not &* 
    db.run(move |c| {
        let result = diesel::insert_into(users::table)
        .values(&*new_user)
        .execute(c)
        .expect("DB ERROR INSERTING");
    json!(result)
    }).await
}

#[put("/users/<id>", format = "json", data = "<user>")]
pub async fn update_users(id: i32, _auth: BasicAuth, db: DbConn, user: Json<User>) -> Value {
    db.run(move |c| {
        let result = diesel::update(users::table.find(id))
        .set((
            users::name.eq(user.name.to_owned()),
            users::email.eq(user.email.to_owned())

        ))
        .execute(c)
        .expect("DB error updating user");
    json!(result)
    }).await
}

#[delete("/users/<id>")]
pub async fn delete_users(id: i32, _auth: BasicAuth, db: DbConn) -> status::NoContent {
    db.run(move |c|{
        diesel::delete(users::table.find(id))
        .execute(c)
        .expect("DB error deleting user");
    status::NoContent
    }).await

}