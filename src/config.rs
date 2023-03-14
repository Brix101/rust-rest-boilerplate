#[derive(clap::Parser)]
pub struct AppConfig {
    #[clap(long, env, default_value = "8000")]
    pub port: u32,

    #[clap(long, env)]
    pub database_url: String,

    #[clap(long, env)]
    pub run_migrations: bool,
}
