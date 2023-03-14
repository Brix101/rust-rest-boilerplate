use std::sync::Arc;

use clap::Parser;
use dotenvy::dotenv;
use rust_rest::{config::AppConfig, utils::connection_pool::ConnectionManager};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let config = Arc::new(AppConfig::parse());

    info!("environment loaded and configuration parsed, initializing Postgres connection and running migrations...");
    let pg_pool = ConnectionManager::new_pool(&config.database_url, config.run_migrations)
        .await
        .expect("could not initialize the database connection pool");

    println!("Hello, world! {:?}", &config.port);

    Ok(())
}
