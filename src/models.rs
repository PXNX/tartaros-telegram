use chrono::prelude::*;
use diesel::{Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};

use crate::schema::users;
use crate::schema::reports;

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

#[derive(Serialize, Queryable, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Report {
    pub id: i32,
    pub author: i32,
    pub date: NaiveDateTime,
    pub user_id: i32,
    pub user_msg: String,
}

#[derive(Insertable, Debug)]
#[table_name="reports"]
pub struct NewReport {
    pub author: i32,
    pub date: NaiveDateTime,
    pub user_id: i32,
    pub user_msg: String,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct InputReport {
    pub author: i32,
    pub user_id: i32,
    pub user_msg: String,
}
