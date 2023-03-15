use std::sync::Arc;

use anyhow::Context;
use clap::Parser;
use dotenvy::dotenv;
use rust_rest::{
    apis::ApplicationController, config::AppConfig, services::ServiceRegister,
    utils::connection_pool::ConnectionManager,
};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let config = Arc::new(AppConfig::parse());

    info!("environment loaded and configuration parsed, initializing Postgres connection and running migrations...");
    let pg_pool = ConnectionManager::new_pool(&config.database_url, config.run_migrations)
        .await
        .expect("could not initialize the database connection pool");

    let service_register = ServiceRegister::new(pg_pool, config.clone());

    info!("migrations successfully ran, initializing axum server...");
    ApplicationController::serve(config.port, &config.cors_origin, service_register)
        .await
        .context("could not initialize application routes")?;

    println!("Hello, world! {:?}", &config.port);

    Ok(())
}
