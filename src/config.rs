#[derive(clap::Parser)]
pub struct AppConfig {
    #[clap(long, env, default_value = "5000")]
    pub port: u16,

    #[clap(long, env)]
    pub database_url: String,

    #[clap(long, env)]
    pub run_migrations: bool,

    #[clap(long, env)]
    pub argon_salt: String,

    #[clap(long, env)]
    pub access_token_secret: String,

    #[clap(long, env)]
    pub refresh_token_secret: String,

    #[clap(long, env)]
    pub cors_origin: String,

    #[clap(long, env)]
    pub seed: bool,
}
