use chrono::NaiveDateTime;

use super::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub creation_timestamp: NaiveDateTime,
    pub reported_message: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: &'a i32,
    pub creation_timestamp: &'a NaiveDateTime,
    pub reported_message: &'a str,
}