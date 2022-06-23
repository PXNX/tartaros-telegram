use rocket::{
    response::status::{Created, NoContent, NotFound},
    serde::json::Json,
};

use diesel::prelude::*;

use tartaros_telegram::{
    models::{User, NewUser},
    schema::users,
    ApiError, PgConnection,
};

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        // State
        .attach(PgConnection::fairing())
        // Routes
        .mount(
            "/users",
            rocket::routes![list, retrieve, create, destroy],
        )
}

#[rocket::get("/")]
async fn list(connection: PgConnection) -> Json<Vec<User>> {
    connection
        .run(|c| users::table.load(c))
        .await
        .map(Json)
        .expect("Failed to fetch users")
}

#[rocket::get("/<id>")]
async fn retrieve(
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
async fn create(
    connection: PgConnection,
    user: Json<NewUser>,
) -> Result<Created<Json<User>>, Json<ApiError>> {
    connection
        .run(move |c| {
            diesel::insert_into(users::table)
                .values(&user.into_inner())
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
async fn destroy(connection: PgConnection, id: i32) -> Result<NoContent, NotFound<Json<ApiError>>> {
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
