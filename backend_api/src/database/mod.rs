mod postgres_db;
pub use postgres_db::check_migrations;
pub use postgres_db::create_pool;

#[cfg(test)]
pub mod tests_db_helper {
    use lazy_static::lazy_static;
    use sqlx::{postgres::PgPoolOptions, PgPool, Postgres};

    lazy_static! {
        pub static ref DB_POOL: PgPool = 
            PgPoolOptions::new()
                .max_connections(10)
                .connect_lazy(&dotenv::var("TESTING_DATABASE_URL").expect("TESTING_DATABASE_URL is not set!"))
                .unwrap();
    }
}
