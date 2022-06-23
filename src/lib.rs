pub mod models;
pub mod schema;

use rocket::serde::{Deserialize, Serialize};

#[macro_use]
extern crate diesel;
use rocket::serde::{Deserialize, Serialize};
use rocket_sync_db_pools::database;

#[database("db")]
pub struct PgConnection(diesel::PgConnection);

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ApiError {
    pub details: String,
}
