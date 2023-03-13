#[derive(clap::Parser)]
pub struct AppConfig {
    #[clap(long, env, default_value = "8000")]
    pub port: u32,
}
