#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::env;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use chrono::Utc;

use diesel::dsl;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use crate::models::{NewUser, User};
use crate::schema::users;
use crate::schema::users::creation_timestamp;

pub mod schema;
mod models;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


pub fn report_user<'a>(conn: &PgConnection, Json(payload): Json<NewUser>)  -> impl IntoResponse {
    use schema::users;

    println!("--- insert");

    let new_user = NewUser{
        id:  payload.id,
        creation_timestamp: &Utc::now().naive_utc(),
        reported_message: payload.reported_message
    };

   let result = create_user(new_user, conn);

    (StatusCode::CREATED, Json(result))


}

pub fn create_user(new_user: NewUser, conn: &PgConnection) -> QueryResult<User> {
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
}

