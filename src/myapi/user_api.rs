


use diesel::ExpressionMethods;
use diesel::prelude::*;
use rocket::serde::json::{json, Json, Value};
use crate::auth::AuthenticatedUser;
use crate::auth::UserAuth;
use crate::models::{User, NewUser};
use crate::auth::create_jwt;
use crate::schema::users;
use super::super::DbConn;
use rocket::http::Status;
use bcrypt::verify;

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
//curl -X GET "http://127.0.0.1:8000/testJWT" -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWJqZWN0Ijo1LCJpYXQiOjE3MTQzMDA0MTYsImV4cCI6MTcxNDMwMTYxNiwicm9sZSI6MX0.4fnX5U1uPM8Oi2Ctqe6be65sHAfknYqhf04xe7d85FE"

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
        _ => json!({ "error": "Access denied" }),
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
pub async fn create_user(auth: AuthenticatedUser, db: DbConn, mut new_user: Json<NewUser>) -> Value {
    match auth.role {
        1 => {
            new_user.hashgen(); //more explicitly .into_inner() not mut, not &*
            db.run(move |c| {
                let result = diesel::insert_into(users::table)
                .values(&*new_user)
                .execute(c)
                .expect("DB ERROR INSERTING");
            json!(result)
            }).await
        },
        _=> json!({"error": "Access Denied"})
    }
    

}


#[put("/users/<id>", format = "json", data = "<user>")]
pub async fn update_users(id: i32, auth: AuthenticatedUser, db: DbConn, user: Json<User>) -> Value {
    let result = match auth.role {
        1 => {
            db.run(move |c| {
                diesel::update(users::table.find(id))
                .set((
                    users::name.eq(user.name.to_owned()),
                    users::email.eq(user.email.to_owned())
        
                ))
                .execute(c)
                .expect("DB error updating user");
            }).await
        },
        _ => {
            db.run(move |c| {
                diesel::update(users::table.find(auth.role))
                .set((
                    users::name.eq(user.name.to_owned()),
                    users::email.eq(user.email.to_owned())
                ))
                .execute(c)
                .expect("DB error updating user");
            }).await
        }
    };
    json!(result)
    
}

#[delete("/users/<id>")]
pub async fn delete_users(id: i32, auth: AuthenticatedUser, db: DbConn) -> Value {
    match auth.role {
        1 => {
            db.run(move |c|{
                diesel::delete(users::table.find(id))
                .execute(c)
                .expect("DB error deleting user");
            json!({"message": "user deleted", "id": id})
            }).await
        },
        _=> json!({"error": "Access Denied"})
    }
}