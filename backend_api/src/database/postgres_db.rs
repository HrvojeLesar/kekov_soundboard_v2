use log::info;
use sqlx::{
    migrate::MigrateDatabase, postgres::PgPoolOptions, Connection, PgConnection, PgPool, Postgres,
};

use crate::error::errors::KekServerError;

pub async fn create_pool() -> Result<PgPool, KekServerError> {
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL is not set!");
    return Ok(PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?);
}

pub async fn check_migrations() -> Result<(), KekServerError> {
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL is not set!");
    if !Postgres::database_exists(&database_url).await? {
        info!("Creating database!");
        Postgres::create_database(&database_url).await?;
    }

    info!("Applying migrations!");

    let mut connection = PgConnection::connect(&database_url).await?;
    sqlx::migrate!()
        .run(&mut connection)
        .await
        .expect("Error while running migrations!");

    return Ok(());
}
