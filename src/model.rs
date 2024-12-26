use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use serde::Serialize;
use tokio_postgres::NoTls;

impl Todo {
    pub fn new(title: String, status: String) -> Self {
        Self { title, status }
    }

    pub async fn load(pool: &Pool<PostgresConnectionManager<NoTls>>) -> Vec<Self> {
        let conn = pool.get().await.unwrap();
        let results = conn
            .query("SELECT title, status from Todos", &[])
            .await
            .expect("Error loading posts .");

        results
            .into_iter()
            .map(|row| Self {
                title: row.get(0),
                status: row.get(1),
            })
            .collect()
    }
}

#[derive(Serialize)]
pub struct Todo {
    title: String,
    status: String,
}

pub enum Status {
    Active,
    Pending,
    Cancelled,
}
