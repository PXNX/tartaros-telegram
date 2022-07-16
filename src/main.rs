use std::env;
use std::net::SocketAddr;

use axum::{Extension, Json, Router};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::routing::{get, post};
use axum_sqlx_tx::Layer;
//use axum_sqlx_tx::{Layer, Tx};
use dotenv::dotenv;
use sqlx::{PgPool, Pool, Postgres, postgres::PgPoolOptions, query, query_as};

use crate::models::{ApiError, InputReport, Report};

mod models;


#[tokio::main]
async fn main() {
   // pretty_env_logger::init();
    dotenv().ok();

    println!("Hello there!");

   let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&*env::var("DATABASE_URL").expect("DATABASE_URL must be provided!")).await.unwrap();

// failed to resolve: could not find `PostgresPool` in `sqlx`
   // let pool = sqlx::PostgresPool::connect(&*env::var("DATABASE_URL").expect("DATABASE_URL must be provided!")).await?;

    let app = Router::new()

        // .layer(axum_sqlx_tx::Layer::<Postgres>::new(pool))
        .route("/", get(redirect_readme))
            .route("/reports", post(report_user))
            .route("/users/:user_id", get(user_by_id))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn redirect_readme() -> Redirect {
    Redirect::to("https://github.com/PXNX/tartaros-telegram#readme")
}


async fn user_by_id(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(user_id): Path<i64>,
) -> Result<Json<Report>, (StatusCode, Json<ApiError>)> {
    query_as!(Report, r#"Select * from reports where user_id = $1 and is_banned=true"#, user_id).fetch_one(&pool)
        .await
        .map(Json)
        .map_err(|e|
            (StatusCode::NOT_FOUND, Json(ApiError {
                details: e.to_string(),
            }))
        )
}

async fn report_user(
    Extension(pool):  Extension<Pool<Postgres>>,
    report: Json<InputReport>,
) -> Result<(StatusCode, Json<Report>), Json<ApiError>> {
    let result = sqlx::query_as!(Report, r#"Insert into reports (user_id, account_id, message) values ($1, $2, $3) returning *"#, report.user_id, 1, report.message).fetch_one(&pool)
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