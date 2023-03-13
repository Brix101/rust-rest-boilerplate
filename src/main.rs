use dotenvy::dotenv;
use rust_rest::core::config::AppConfig;
use std::sync::Arc;

fn main() {
    dotenv().ok();
    let config = Arc::new(AppConfig::new());

    println!("Hello, world! {:?}", &config.port);
}
