#[macro_use] extern crate rocket;

use diesel::{dsl::Limit, ExpressionMethods};
use diesel::prelude::*;
use rocket::{fairing, http::Status, response::status, serde::json::{json, Value}};
use rocket_sync_db_pools::database;
use models::User;
use auth::BasicAuth;
use schema::users;


mod auth;
mod models;
mod schema;
//\


#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);



//CRUD OPERATIONS {GET: list existing, GET: show single, POST: create new, PUT: update existing, DELETE: delete existing} DUMMY VALUES FOR NOW HARDCODED DUH

#[get("/users")]
async fn get_users(_auth: BasicAuth, db: DbConn) -> Value {
    db.run(|c| {
       let users =  users::table.order(users::id.desc()).limit(1000).load::<User>(c).expect("DB ERROR");
       json!(users)
    }).await
    
}
#[get("/users/<id>")]
fn view_user(id: i32, _auth: BasicAuth ) -> Value {
    json!({"id": id, "name": "John Doe", "email": "john@doe.com"})
}
#[post("/users", format = "json")]
fn create_user(_auth: BasicAuth) -> Value {
    json!({"id": 3, "name": "John Doe", "email": "john@doe.com"})
}
#[put("/users/<id>", format = "json")]
fn update_users(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "John Doe", "email": "john@doe.com"})
}
#[delete("/users/<_id>")]
fn delete_users(_id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
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

            ])
        .attach(DbConn::fairing()) //check if it can launch
        .launch()
        .await {
        println!("Failed to launch Rocket: {}", e);
    }
}
