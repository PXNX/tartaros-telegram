#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::env;
use std::error::Error;
use std::fmt::format;

use std::sync::atomic::{AtomicU64, Ordering};



use chrono::prelude::Utc;
use diesel::prelude::*;
use dotenv::dotenv;
use lazy_static::lazy_static;
use rocket::{Request, request, response::status::{Created, NoContent, NotFound}, serde::json::Json};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::response::Redirect;
use teloxide::{
    dispatching::{
        dialogue::{self, InMemStorage},
        UpdateHandler,
    },
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};
use teloxide::prelude::*;

use tartaros_telegram::{
    ApiError,
    models::{NewUser, User},
    PgConnection, schema::users,
};

lazy_static! {
    static ref BOT: AutoSend<Bot> = Bot::from_env().auto_send();
}





async fn bot() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    println!("aaauwwwuaa");




    println!("aaauoooooooowwwuaa");

 let handler = Update::filter_callback_query().branch(dptree::endpoint(callback_handler));

/*  let handler =  Update::filter_message().branch(dptree::endpoint(
        |msg: Message| async move {
            let previous = 55;
            BOT.send_message(msg.chat.id, format!("I received {} messages in total.", previous))
                .await?;
            respond(())
        },
    )); */

    Dispatcher::builder(&*BOT, handler).build().setup_ctrlc_handler().dispatch().await;


    println!("aaaapppppa");
    Ok(())
}


#[rocket::launch]
async  fn rocket() -> _ {
    // pretty_env_logger::init();
    dotenv().ok();
    println!("Hello there!");
    //  log::info!("Starting Teloxide...");




    //  log::info!("Starting Rocket...");


   tokio::spawn(async {
    bot().await;
  });


    println!("aaauuaa");



    let rock =  rocket::build()
        .attach(PgConnection::fairing())
        .mount("/", rocket::routes![redirect_readme])
        .mount("/reports", rocket::routes![report_user])
        .mount("/users", rocket::routes![all_users, user_by_id,  unban_user]);

    // .launch()
    //         .await;

    println!("aaaaa");

     rock
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
async fn report_user(
    connection: PgConnection,
    user: Json<NewUser>,
    _token: Token,
) -> Result<Created<Json<i32>>, Json<ApiError>> {
    let keyboard = make_keyboard();
    BOT.send_message(ChatId(-1001758396624), format!("Report\n\nUser: {}\n\nMessage: {}", user.id, user.msg)).reply_markup(keyboard).await.map(|a| Created::new("/").body(Json(a.id)))
        .map_err(|e| {
            Json(ApiError {
                details: e.to_string(),
            })
        })



    /*
    connection
        .run(move |c| unsafe {



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
        }) */
}

async fn ban_user(connection: PgConnection,
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
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(version) = q.data {
        let text = format!("You chose: {version}");


        match q.message {
            Some(Message { id, chat, .. }) => {
                BOT.edit_message_text(chat.id, id, text).await?;
            }
            None => {
                if let Some(id) = q.inline_message_id {
                    BOT.edit_message_text_inline(id, text).await?;
                }
            }
        }


        log::info!("You chose: {}", version);
    }

    Ok(()) //     respond(())
}

fn make_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    let debian_versions = [
        "Buzz", "Rex", "Bo", "Hamm", "Slink", "Potato", "Woody", "Sarge", "Etch", "Lenny",
        "Squeeze", "Wheezy", "Jessie", "Stretch", "Buster", "Bullseye",
    ];

    for versions in debian_versions.chunks(3) {
        let row = versions
            .iter()
            .map(|&version| InlineKeyboardButton::callback(version.to_owned(), version.to_owned()))
            .collect();

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}