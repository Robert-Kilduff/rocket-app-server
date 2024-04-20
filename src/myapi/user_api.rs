

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
//---test login and persist

#[post("/login", data = "<login>")]
pub async fn begin_auth_session(login: Json<UserAuth>, db: DbConn) -> Result<Json<rocket::serde::json::Value>, Status> {
    let user = db.run(move |c| {
        users::table.filter(users::name.eq(&login.username))
        .first::<User>(c).ok()
    }).await;

    match user {
        Some(user) if user.passhash.unwrap() == login.password => { 
            // Create JWT 
            match create_jwt(&user.id, user.role) {
                Ok(token) => Ok(json!({"token": token})),
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
pub async fn create_user(_auth: BasicAuth, db: DbConn, new_user: Json<NewUser>) -> Value {
    db.run(|c| {
        let result = diesel::insert_into(users::table)
        .values(new_user.into_inner())
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