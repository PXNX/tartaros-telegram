use rocket::serde::{Deserialize, Serialize};
use chrono::prelude::*;
use diesel::{Insertable, Queryable};

use crate::schema::users;

#[derive(Serialize, Queryable, Debug)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub msg: String,
    pub date: NaiveDateTime
}

#[derive(Deserialize, Insertable, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct NewUser {
    pub id: i32,
    pub msg: String
}
