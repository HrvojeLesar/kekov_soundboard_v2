mod postgres_db;
pub use postgres_db::check_migrations;
pub use postgres_db::create_pool;

#[cfg(test)]
pub mod tests_db_helper {
    use lazy_static::lazy_static;
    use sqlx::{postgres::PgPoolOptions, PgPool, Postgres, PgConnection, Connection};

    pub async fn db_connection() -> PgConnection {
        return PgConnection::connect(&dotenv::var("TESTING_DATABASE_URL").expect("TESTING_DATABASE_URL is not set!")).await.unwrap();
    }
}
