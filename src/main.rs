use std::sync::Arc;

use anyhow::Context;
use clap::Parser;
use dotenvy::dotenv;

use tracing::info;

use rest_api::{AppConfig, ApplicationServer, Database, Logger};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    Logger::init();

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
