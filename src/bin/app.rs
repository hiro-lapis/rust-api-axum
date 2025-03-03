use std::net::{Ipv4Addr, SocketAddr};
use axum::{extract::State, http::StatusCode, routing::get, Router};
use tokio::net::TcpListener;
use anyhow::Result;
use sqlx::{postgres::PgConnectOptions, PgPool};

// handler
async fn lapis() -> &'static str {
    "hiro lapis"
}

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn health_check_db(State(db): State<PgPool>) -> StatusCode {
    let connection_result = sqlx::query("SELECT 1").fetch_one(&db).await;
    match connection_result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let database_cfg = DatabaseConfig {
        host: std::env::var("DATABASE_HOST").unwrap(),
        port: std::env::var("DATABASE_PORT").unwrap().parse::<u16>().unwrap(),
        username: std::env::var("DATABASE_USERNAME").unwrap(),
        password: std::env::var("DATABASE_PASSWORD").unwrap(),
        database: std::env::var("DATABASE_NAME").unwrap(),
    };

    let conn_pool = adapter::database::connect_database_with(database_cfg);
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/lapis", get(lapis))
        .route("/health/db", get(health_check_db))
        .with_state(conn_pool);
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on {}", addr);

    // axum::serve(listener, app).await.unwrap();
    Ok(axum::serve(listener, app).await?)
}

#[tokio::test]
async fn health_check_works() {
    let status_code = health_check().await;
    assert_eq!(status_code, StatusCode::OK);
}

struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl From<DatabaseConfig> for PgConnectOptions {
    fn from(cfg: DatabaseConfig) -> Self {
        Self::new()
            .host(&cfg.host)
            .port(cfg.port)
            .username(&cfg.username)
            .password(&cfg.password)
            .database(&cfg.database)
    }
}

// fn connect_database_with(cfg: DatabaseConfig) -> PgPool {
//     PgPool::connect_lazy_with(cfg.into())
// }

#[sqlx::test]
async fn health_check_db_works (pool: sqlx::PgPool) {
    let status_code = health_check_db(State(pool)).await;
    assert_eq!(status_code, StatusCode::OK);
}