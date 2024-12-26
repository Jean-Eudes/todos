/* KATA du jour
   Ecrire une API pour écrire dans la base
   Enregister des elements en base de données
   Ecrire lire de la base
*/
mod model;
mod schema;

use crate::model::TodosToPersist;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{debug_handler, Json, Router};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use diesel::{r2d2, Connection};
use std::env;
use axum::extract::State;

#[derive(Clone)]
struct AppState {
    pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

#[tokio::main]
async fn main() {
    let database_url = "postgres://omc_projet:omc_projet@localhost:5432/todos";
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let state = AppState { pool };

    // build our application with a route
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
async fn handler2(State(state): State<AppState>) -> Json<Vec<TodosToPersist>> {
    let mut connection = state
        .pool
        .get()
        .expect("Failed to get a connection from the pool");
    let vec = TodosToPersist::load(&mut connection);
    Json(vec)
}
