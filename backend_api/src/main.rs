use std::{
    fs::File,
    io::BufReader,
    sync::{Arc, Mutex}, time::UNIX_EPOCH, collections::HashMap,
};

use actix::Actor;
use actix_web::{web::Data, App, HttpServer};
use routes::{routes_config, not_found::not_found};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

use dotenv::dotenv;
use snowflake::SnowflakeIdGenerator;
use tokio::sync::RwLock;
use ws::{ws_server::ControlsServer, ws_session::WsSessionCommChannels};

mod database;
mod discord_client_config;
mod error;
mod middleware;
mod models;
mod oauth_client;
mod routes;
mod utils;
mod ws;

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

    let snowflake_thread_id = Arc::new(Mutex::new(0));

    let controls_server = Data::new(ControlsServer::new());
    let ws_channels: Data<WsSessionCommChannels> = Data::new(RwLock::new(HashMap::new()));

    return HttpServer::new(move || {
        // Per thread snowflake generator
        let snowflakes;
        {
            let id_arc = snowflake_thread_id.clone();
            let mut lock = id_arc.lock().unwrap();
            let epoch = UNIX_EPOCH + std::time::Duration::from_millis(1640991600000); // epoch start time 01.01.2022. 00:00
            snowflakes = Data::new(Mutex::new(SnowflakeIdGenerator::with_epoch(*lock, 1, epoch)));
            *lock += 1;
        }

        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(oauth.clone()) // oauth2::basic::BasicClient
            .app_data(pool.clone())
            .app_data(controls_server.clone())
            .app_data(ws_channels.clone())
            .app_data(snowflakes)
            .configure(routes_config)
            .default_service(actix_web::web::to(not_found))
    })
    // .bind_rustls(&bind_address, config)?
    .bind(bind_address)?
    // .workers(1) 
    .run()
    .await;
}
