use std::sync::Arc;

use anyhow::Context;
use clap::Parser;
use dotenvy::dotenv;

use budgetto_api::{config::AppConfig, database::Database, server::ApplicationServer};
use tracing::info;

mod logger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    logger::init();

    let config = Arc::new(AppConfig::parse());

    info!("environment loaded and configuration parsed, initializing Postgres connection and running migrations...");
    let db = Database::connect(&config.database_url, config.run_migrations)
        .await
        .expect("could not initialize the database connection pool");

    ApplicationServer::serve(config, db)
        .await
        .context("could not initialize application routes")?;

    Ok(())
}
