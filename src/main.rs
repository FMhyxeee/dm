use axum::{routing::get, Router};
use dm::{index, list_all};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:3000";

    let listener = TcpListener::bind(addr).await.unwrap();

    info!("Server is running on {}", addr);

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://hyx:hyx@localhost:5432/dm")
        .await?;

    let app = Router::new()
        .route("/", get(index))
        .route("/list_all", get(list_all))
        .with_state(pool);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
