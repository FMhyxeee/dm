use axum::{routing::get, Router};
use dm::index;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:3000";

    let listener = TcpListener::bind(addr).await.unwrap();

    info!("Server is running on {}", addr);

    let app = Router::new().route("/", get(index));

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
