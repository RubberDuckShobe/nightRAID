use ezsockets::Server;
use sqlx::postgres::PgPool;
use tracing::info;

mod db;
pub mod errors;
pub mod game;
mod ws;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    info!("Connecting to DB...");
    // init DB, run migrations
    let pool = PgPool::connect(&dotenvy::var("DATABASE_URL").unwrap()).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    info!("Starting server");
    let (server, _) = Server::create(|_server| ws::GameServer { pool });
    ezsockets::tungstenite::run(server, "127.0.0.1:7331")
        .await
        .unwrap();

    Ok(())
}
