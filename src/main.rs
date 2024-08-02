#[macro_use] 
extern crate rocket;
extern crate dotenv;

use rocket::figment::{Figment, providers::{Format, Toml}};
use rocket::serde::json::{json, Value};
use rocket_sync_db_pools::database;
use dotenv::dotenv;
use std::env;
use bcrypt::{hash, verify, DEFAULT_COST};

use rocket::Config;

//use serde_json::json;
mod myapi;
mod auth;
mod models;
mod schema;
mod services;
//\


#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);


//curl 127.0.0.1:8000/users -H 'Authorization: Basic Zm9vOmJhcg==' -d '{"name": "HELPME", "email": "AMINEW@Konto.com", "role": "2"}' -H 'Content-type: application/json' -X PUT


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
    dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET_KEY")
        .expect("JWT_SECRET_KEY must be set");
    println!("JWT Secret: {}", jwt_secret);
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
    let test_password = "password";
    let hashed = match hash(test_password, DEFAULT_COST) {
        Ok(h) => h,
        Err(e) => panic!("Error hashing password: {}", e),
    };
    println!("Hashed password: {}", hashed);
    match verify(test_password, &hashed) {
        Ok(true) => println!("Password verified!"),
        Ok(false) => println!("Password verification failed!"),
        Err(e) => panic!("Error verifying password: {}", e),
    }

    let figment = Figment::from(rocket::Config::default())
    .merge(("address", "0.0.0.0"))
    .merge(("port", 8000))
    .merge(Toml::file("Rocket.toml").nested());

    if let Err(e) = rocket::custom(figment)
     // bind to all network interfaces
        .mount("/", routes![
            myapi::user_api::begin_auth_session,

            myapi::user_api::test_jwt,

            myapi::habit_api::get_habits_controller,
            myapi::habit_api::view_habit_controller,
            myapi::habit_api::create_habit_controller,
            myapi::habit_api::delete_habit_controller,
            myapi::habit_api::update_habit_controller,

            myapi::user_api::get_users_controller,
            myapi::user_api::view_user_controller,
            myapi::user_api::update_users_controller,
            myapi::user_api::create_user_controller,
            myapi::user_api::delete_user_controller,

            myapi::task_api::get_tasks_controller,
            myapi::task_api::view_task_controller,
            myapi::task_api::create_task_controller,
            myapi::task_api::delete_task_controller,
            myapi::task_api::update_task_controller,
            

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
