use serde::Serialize;
use std::env;

#[derive(Serialize)]
pub struct AppConfig {
    pub port: u16,
}

impl AppConfig {
    pub fn new() -> Self {
        // for key in AppConfig::from(value) {
        //     println!("{:?}", key);
        // }
        // for (key,value) in env::vars{
        //
        // }
        AppConfig {
            port: env::var("PORT")
                .unwrap_or("8000".to_string())
                .parse()
                .unwrap(),
        }
    }
}
