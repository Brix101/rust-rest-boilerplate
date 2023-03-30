use std::sync::Arc;

use anyhow::Context;
use axum_sqlx_rest_api_with_jwt::{
    config::AppConfig, database::Database, logger, server::ApplicationServer,
};
use clap::Parser;
use dotenvy::dotenv;

use tracing::info;

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
