use chrono::prelude::*;
use diesel::{Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Serialize, Queryable, Insertable, Debug)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub msg: String,
    pub date: NaiveDateTime,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct NewUser {
    pub id: i32,
    pub msg: String,
}
