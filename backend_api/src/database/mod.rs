mod postgres_db;
pub use postgres_db::check_migrations;
pub use postgres_db::create_pool;

#[cfg(test)]
pub mod tests_db_helper {
    use sqlx::{Connection, PgConnection, Postgres, migrate::MigrateDatabase};

    pub async fn db_connection() -> PgConnection {
        let database_url =
            dotenv::var("TESTING_DATABASE_URL").expect("TESTING_DATABASE_URL is not set!");
        if !Postgres::database_exists(&database_url).await.unwrap() {
            Postgres::create_database(&database_url).await.unwrap();
        }

        let mut connection = PgConnection::connect(&database_url).await.unwrap();

        sqlx::migrate!().run(&mut connection).await.unwrap();

        return connection;
    }
}
