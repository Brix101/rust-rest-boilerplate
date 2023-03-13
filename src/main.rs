use clap::Parser;
use dotenvy::dotenv;
use rust_rest::config::AppConfig;
use std::sync::Arc;

fn main() {
    dotenv().ok();
    let config = Arc::new(AppConfig::parse());

    println!("Hello, world! {:?}", &config.port);
}
