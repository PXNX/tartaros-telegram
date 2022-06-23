#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::env;

use diesel::dsl;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


pub fn report_user<'a>(conn: &PgConnection, id: &'a i32, reported_message: &'a str) {
    use schema::users;

    println!("--- insert");

    diesel::insert_into(users::table)
        .values((
            users::id.eq(id),
            users::creation_timestamp.eq(dsl::now),
            users::reported_message.eq(reported_message)
        ))
        .execute(&*conn)
        .expect(&*format!("Error saving new user {}", id));
}