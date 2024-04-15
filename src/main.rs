#[macro_use] extern crate rocket;

use diesel::connection::TransactionDepthChange;
use diesel::{dsl::Limit, ExpressionMethods};
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::{fairing, http::Status, response::status, serde::json::{json, Value}};
use rocket_sync_db_pools::database;
use models::{User, NewUser};
use auth::BasicAuth;
use schema::users;
//use serde_json::json;


mod auth;
mod models;
mod schema;
//\


#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);


//curl http://127.0.0.1:8000/users/1 -H 'Authorization: Basic Zm9vOmJhcg==' -d '{"name": "foo", "email": "bar.com", "role": "2", "id": 1, "created_at": "2024-01-01"}' -H 'Content-type: application/json' -X PUT
//CRUD OPERATIONS {GET: list existing, GET: show single, POST: create new, PUT: update existing, DELETE: delete existing} DUMMY VALUES FOR NOW HARDCODED DUH

#[get("/users")]
async fn get_users(_auth: BasicAuth, db: DbConn) -> Value {
    db.run(|c| {
       let users =  users::table.order(users::id.desc())
       .limit(1000).load::<User>(c)
       .expect("DB ERROR");
       json!(users)
    }).await
    
}
#[get("/users/<id>")]
async fn view_user(id: i32, _auth: BasicAuth , db: DbConn) -> Value {
    db.run(move |c| {
        let user = users::table.find(id)
        .get_result::<User>(c)
        .expect("DB error selecting user");
        json!(user)
    }).await
}
#[post("/users", format = "json", data = "<new_user>")]
async fn create_user(_auth: BasicAuth, db: DbConn, new_user: Json<NewUser>) -> Value {
    db.run(|c| {
        let result = diesel::insert_into(users::table)
        .values(new_user.into_inner())
        .execute(c)
        .expect("DB ERROR INSERTING");
    json!(result)
    }).await
}

#[put("/users/<id>", format = "json", data = "<user>")]
async fn update_users(id: i32, _auth: BasicAuth, db: DbConn, user: Json<User>) -> Value {
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
async fn delete_users(id: i32, _auth: BasicAuth, db: DbConn) -> status::NoContent {
    db.run(move |c|{
        diesel::delete(users::table.find(id))
        .execute(c)
        .expect("DB error deleting user");
    status::NoContent
    }).await

}

//ERRORS

#[catch(404)]
fn not_found() -> Value {
    json!("Not found")
}
#[catch(401)]
fn unauthorized() -> Value {
    json!("Invalid/Missing Authorization")
}
#[catch(422)]
fn unprocessable_entity() -> Value {
    json!("unprocessable entity")
}
#[rocket::main]
async fn main() {
    if let Err(e) = rocket::build()
        .mount("/", routes![
            get_users,
            view_user,
            create_user,
            update_users,
            delete_users

            ])
            .register("/", catchers![
                not_found,
                unauthorized,
                unprocessable_entity,

            ])
        .attach(DbConn::fairing()) //check if it can launch
        .launch()
        .await {
        println!("Failed to launch Rocket: {}", e);
    }
}
