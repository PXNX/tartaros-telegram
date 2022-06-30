#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::env;

use chrono::prelude::Utc;
use diesel::prelude::*;
use dotenv::dotenv;
use rocket::{Request, request, response::status::{Created, NoContent, NotFound}, serde::json::Json};
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::request::{FromRequest, Outcome};

use tartaros_telegram::{
    ApiError,
    models::{NewUser, User},
    PgConnection, schema::users,
};

#[rocket::launch]
fn rocket() -> _ {
    println!("hello there!");
    dotenv().ok();
    rocket::build()
        .attach(PgConnection::fairing())
        .mount(
            "/users",
            rocket::routes![all_users, user_by_id, ban_user, unban_user],
        )
        .mount("/", redirect_readme)
}

#[rocket::get("/")]
async fn redirect_readme() -> Redirect {
    Redirect::to("https://github.com/PXNX/tartaros-telegram#readme")
}

#[rocket::get("/")]
async fn all_users(connection: PgConnection) -> Json<Vec<User>> {
    connection
        .run(|c| users::table.load(c))
        .await
        .map(Json)
        .expect("Failed to fetch users")
}

#[rocket::get("/<id>")]
async fn user_by_id(
    connection: PgConnection,
    id: i32,
) -> Result<Json<User>, NotFound<Json<ApiError>>> {
    connection
        .run(move |c| users::table.filter(users::id.eq(id)).first(c))
        .await
        .map(Json)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}

#[rocket::post("/", data = "<user>")]
async fn ban_user(
    connection: PgConnection,
    user: Json<NewUser>,
    _token: Token,
) -> Result<Created<Json<User>>, Json<ApiError>> {
    connection
        .run(move |c| {
            diesel::insert_into(users::table)
                .values(User {
                    id: user.id,
                    msg: String::from(&user.msg),
                    date: Utc::now().naive_utc(),
                })
                .get_result(c)
        })
        .await
        .map(|a| Created::new("/").body(Json(a)))
        .map_err(|e| {
            Json(ApiError {
                details: e.to_string(),
            })
        })
}


#[rocket::delete("/<id>")]
async fn unban_user(connection: PgConnection, id: i32, _token: Token) -> Result<NoContent, NotFound<Json<ApiError>>> {
    connection
        .run(move |c| {
            let affected = diesel::delete(users::table.filter(users::id.eq(id)))
                .execute(c)
                .expect("Connection is broken");
            match affected {
                1 => Ok(()),
                0 => Err("NotFound"),
                _ => Err("???"),
            }
        })
        .await
        .map(|_| NoContent)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}

struct Token(String);

#[derive(Debug)]
enum ApiTokenError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ApiTokenError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("token");
        return match token {
            Some(token) => {
                let actual = env::var("HASH").expect("$HASH is not set");

                if actual == token {
                    return Outcome::Success(Token(token.to_string()));
                }

                Outcome::Failure((Status::Unauthorized, ApiTokenError::Invalid))
            }
            None => Outcome::Failure((Status::Unauthorized, ApiTokenError::Missing)),
        };
    }
}