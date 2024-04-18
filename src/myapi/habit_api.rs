use diesel::ExpressionMethods;
use diesel::prelude::*;
use rocket::serde::json::{json, Json, Value};
use rocket::response::status;
use crate::models::{User, NewUser};
use crate::auth::BasicAuth;
use crate::schema::users;
use super::super::DbConn;

#[get("/habits")]
pub async fn get_users(_auth: BasicAuth, db: DbConn) -> Value {
    db.run(|c| {
       let users =  users::table.order(users::id.desc())
       .limit(1000).load::<User>(c)
       .expect("DB ERROR");
       json!(users)
    }).await
    
}