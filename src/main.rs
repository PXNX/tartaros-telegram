#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rocket;

use std::{env, future};
use std::borrow::Borrow;
use std::error::Error;
use std::fmt::format;
use std::process::Termination;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};

use chrono::prelude::Utc;
use diesel::prelude::*;
use dotenv::dotenv;
use lazy_static::lazy_static;
use rocket::{response::status::{Created, NoContent, NotFound}, serde::json::Json, State};
use rocket::fairing::AdHoc;
use rocket::futures::executor;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::request::Outcome;
use rocket::response::Redirect;
use serde::Deserialize;
use teloxide::{dispatching::{
    dialogue::{self, InMemStorage},
    UpdateHandler,
}, prelude::*, RequestError, types::{InlineKeyboardButton, InlineKeyboardMarkup}, utils::command::BotCommands};
use teloxide::prelude::*;

use tartaros_telegram::{
    ApiError,
    models::{NewUser, User},
    PgConnection, schema::users,
};
use tartaros_telegram::models::{InputReport, NewReport, Report};
use tartaros_telegram::schema::reports;

/*
#[derive(Deserialize)]
struct Config {
    api_key: String,
}

struct ApiKey<'r>(&'r str);

#[derive(Debug)]
enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {

        // Retrieve the config state like this
        let config = req.rocket().state::<Config>().unwrap();

        fn is_valid(key: &str, api_key: &str) -> bool {
            key == api_key
        }

        match req.headers().get_one("Authorization") {
            None => Outcome::Failure((Status::Unauthorized, ApiKeyError::Missing)),
            Some(key) if is_valid(key, &config.api_key) => Outcome::Success(ApiKey(key)),
            Some(_) => Outcome::Failure((Status::Unauthorized, ApiKeyError::Invalid)),
        }
    }
}

*/


struct MyState {
    bbot: AutoSend<Bot>,
}

/*
struct Item(AutoSend<Bot>);

impl<'a, 'r> FromRequest<'a, 'r> for Item {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Item, ()> {
        request.guard::<State<MyState>>()
            .map(|my_config| Item(my_config.bbot.clone()))
    }
}*/

/*
impl<'a, 'r> FromRequest<'a, 'r> for AutoSend<Bot> {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match request.real_ip() {
            Some(bot:Bot) => Outcome::Success(AutoSend(bot)),
            None => Outcome::Failure((Status::from_code(401).unwrap(), ()))
        }
    }
}*/

#[rocket::launch]
async fn rocket() -> _ {
    pretty_env_logger::init();
    dotenv().ok();

    println!("Hello there!");

    //  let mut db: Option<PgConnection> = None;

    let bot: AutoSend<Bot> = Bot::from_env().auto_send();

    let state = MyState {
        bbot: bot.clone()
    };

    log::info!("Starting Rocket...");
    let rocket = rocket::build()
        .manage(state)
        .attach(PgConnection::fairing())
        .attach(AdHoc::on_liftoff("Startup Check", |rocket| {
            Box::pin(async move {

                log::info!("Starting Teloxide...");
                let db = PgConnection::get_one(rocket).await.unwrap();


                let handler = Update::filter_callback_query().branch(dptree::endpoint(callback_handler));

                Dispatcher::builder(bot.clone(), handler)
                    .dependencies(dptree::deps![db])
                    .build()
                    .setup_ctrlc_handler()
                    .dispatch()
                    .await;
                log::info!("Started Teloxide.");
            })
        }))
        .mount("/", rocket::routes![redirect_readme])
        .mount("/reports", rocket::routes![report_user])
        .mount("/users", rocket::routes![all_users, user_by_id,  unban_user]);

    log::info!("Started Rocket.");

    rocket
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

#[rocket::post("/", data = "<report>")]
async fn report_user(
    connection: PgConnection,
    report: Json<InputReport>,
    state: &State<MyState>,
) -> Result<Created<Json<Report>>, Json<ApiError>> {
    let result: QueryResult<Report> = connection
        .run(move |c| {
            diesel::insert_into(reports::table)
                .values::<NewReport>(NewReport {
                    author: report.author,
                    date: Utc::now().naive_utc(),
                    user_id: report.user_id,
                    user_msg: String::from(&report.user_msg),
                })
                .get_result::<Report>(c)
        })
        .await;

    match result {
        Ok(res) => {
            let keyboard = InlineKeyboardMarkup::new(vec![vec![
                InlineKeyboardButton::callback("Ban user 🚫", &res.id.to_string())
            ]]);

            state.inner().bbot.send_message(ChatId(-1001758396624),
                                            format!("Report {}\n\nUser: {}\n\nMessage: {}",
                                                    &res.id, &res.user_id, &res.user_msg))
                .reply_markup(keyboard).await.expect("Failed to send message");


            return Ok(Created::new("/").body(Json(res)));
        },
        Err(e) => return Err(Json(ApiError {
            details: e.to_string()
        }))
    }
}

trait Block {
    fn wait(self) -> <Self as future::Future>::Output
        where Self: Sized, Self: future::Future
    {
        executor::block_on(self)
    }
}

impl<F, T> Block for F
    where F: future::Future<Output=T>
{}

async fn ban_user(connection: &PgConnection,
                  user: NewUser, ) -> Result<Created<Json<User>>, Json<ApiError>> {
    connection
        .run(move |c| diesel::insert_into(users::table)
            .values(User {
                id: user.id,
                msg: String::from(&user.msg),
                date: Utc::now().naive_utc(),
            })
            .get_result(c)
        )
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


async fn callback_handler(
    q: CallbackQuery,
    connection: &PgConnection, bot: AutoSend<Bot>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(report_id) = q.data {
        match q.message {
            Some(Message { id, chat, .. }) => {
                let report = report_by_id(&connection, i32::from_str(&*report_id).unwrap()).await;


                ban_user(&connection, NewUser {
                    id: report.as_ref().unwrap().user_id,
                    msg: String::from(&report.as_ref().unwrap().user_msg),
                }).await;


                bot.edit_message_reply_markup(chat.id, id).await?;
            }

            _ => {}
        }
    }

    Ok(()) //     respond(())
}

async fn report_by_id(
    connection: &PgConnection,
    id: i32,
) -> Result<Report, NotFound<Json<ApiError>>> {
    connection
        .run(move |c| reports::table.filter(reports::id.eq(id)).first(c))
        .await

        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}
