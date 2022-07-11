use std::env;
use std::net::SocketAddr;

use axum::{Json, Router};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::routing::{get, post};
use axum_sqlx_tx::Tx;
use dotenv::dotenv;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions, query, query_as};

use crate::models::{ApiError, InputReport, Report};

mod models;


#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv().ok();

    println!("Hello there!");

   let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&*env::var("DATABASE_URL").expect("DATABASE_URL must be provided!")).await?;

// failed to resolve: could not find `PostgresPool` in `sqlx`
   // let pool = sqlx::PostgresPool::connect(&*env::var("DATABASE_URL").expect("DATABASE_URL must be provided!")).await?;

    let app = Router::new()
        .layer(axum_sqlx_tx::Layer::<Postgres>::new(pool))
        .route("/", get(redirect_readme)
            .route("/reports", post(report_user))
            .route("/users/:user_id", get(user_by_id)),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn redirect_readme() -> Redirect {
    Redirect::to("https://github.com/PXNX/tartaros-telegram#readme")
}


async fn user_by_id(
    mut tx: Tx<Postgres>,
    id: i64,
) -> Result<Json<Report>, (StatusCode, Json<ApiError>)> {
    query_as!(Report, r#"Select * from reports where user_id = $1 and is_banned=true"#, id).fetch_one(tx)
        .await
        .map(Json)
        .map_err(|e|
            (StatusCode::NOT_FOUND, Json(ApiError {
                details: e.to_string(),
            }))
        )
}

async fn report_user(
    mut tx: Tx<Postgres>,
    report: Json<InputReport>,
) -> Result<(StatusCode, Json<Report>), Json<ApiError>> {
    let result = sqlx::query_as!(Report, r#"Insert into reports (user_id, account_id, message) values ($1, $2, $3) returning *"#, report.user_id, 123, report.message).execute(tx)
        .await;

    return match result {
        Ok(res) => {
            Ok((StatusCode::CREATED, Json(res)))
        }
        Err(e) => Err(Json(ApiError {
            details: e.to_string()
        }))
    };
}