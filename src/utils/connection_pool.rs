use anyhow::{Context, Ok};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, Row};
use tracing::info;

pub type ConnectionPool = Pool<Postgres>;

pub struct ConnectionManager;

impl ConnectionManager {
    pub async fn new_pool(
        connection_string: &str,
        run_migrations: bool,
    ) -> anyhow::Result<ConnectionPool> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(connection_string)
            .await
            .context("error while initializing the database connection pool")?;

        if run_migrations {
            info!("migrations enabled, running...");
            sqlx::migrate!()
                .run(&pool)
                .await
                .context("error while running database migrations")?;
        }

        // get all database table
        let result = sqlx::query(
            "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public' AND table_name NOT LIKE '_sqlx_%'",
        )
        .fetch_all(&pool)
        .await.unwrap();

        for (idx, row) in result.iter().enumerate() {
            println!("[{}]: {:?}", idx + 1, row.get::<String, &str>("table_name"));
        }
        Ok(pool)
    }
}
