use std::{fs::File, io::BufReader};

use actix_web::{
    App, HttpServer, web::Data
};
use routes::auth::{auth_callback, revoke_token, start_discord_oauth};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

use dotenv::dotenv;

mod database;
mod discord_client_config;
mod error;
mod middleware;
mod oauth_client;
mod routes;
mod utils;

// #[cfg(debug_assertions)]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();
    let cert_file = &mut BufReader::new(File::open("127.0.0.1.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("127.0.0.1-key.pem").unwrap());

    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();

    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    let config = config.with_single_cert(cert_chain, keys.remove(0)).unwrap();

    let bind_address = format!(
        "localhost:{}",
        std::env::var("PORT").unwrap_or("8080".to_string())
    );

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let oauth = Data::new(oauth_client::OAuthClient::new());

    database::check_migrations()
        .await
        .expect("Error occurred while running migrations!");

    let pool = Data::new(
        database::create_pool()
            .await
            .expect("Failed to create database connection pool!"),
    );

    return HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(oauth.clone()) // oauth2::basic::BasicClient
            .app_data(pool.clone())
            .service(start_discord_oauth)
            .service(auth_callback)
            .service(revoke_token)
    })
    // .bind_rustls(&bind_address, config)?
    .bind(bind_address)?
    .run()
    .await;
}
