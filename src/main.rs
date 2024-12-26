/* KATA du jour
   Ecrire une API pour écrire dans la base
   Enregister des elements en base de données
   Ecrire lire de la base
*/
mod model;

use crate::model::Todo;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{debug_handler, Json, Router};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use serde::Deserialize;
use std::sync::Arc;
use tokio_postgres::{Client, NoTls};
use std::str::FromStr;

#[derive(Clone)]
struct AppState {
    pool: Pool<PostgresConnectionManager<NoTls>>,
}

#[tokio::main]
async fn main() {
    let database_url = "postgres://omc_projet:omc_projet@localhost:5432/todos";
    let config = tokio_postgres::config::Config::from_str(database_url).unwrap();
    // build our application with a route
    let pg_mgr = PostgresConnectionManager::new(config, NoTls);

    let pool = match Pool::builder()
        .max_size(30)
        .build(pg_mgr).await {
        Ok(pool) => pool,
        Err(e) => panic!("builder error: {e:?}"),
    };

    let state = AppState {
        pool
    };

    let app = Router::new()
        .route("/", get(handler2))
        .route("/", post(handler))
        .with_state(state);
    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler(title: String) -> StatusCode {
    StatusCode::CREATED
}

#[debug_handler]
async fn handler2(State(state): State<AppState>) -> Json<Vec<Todo>> {
    let rows = Todo::load(&state.pool).await;
    Json(rows)
}
