use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::UNIX_EPOCH,
};

use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use env::check_required_env_variables;
use routes::{not_found::not_found, routes_config};

use dotenv::dotenv;
use snowflake::SnowflakeIdGenerator;
use tokio::sync::RwLock;
use utils::cache::{create_authorized_user_cache, create_user_guilds_cache};
use ws::{ws_server::ControlsServer, ws_session::WsSessionCommChannels};

mod database;
mod discord_client_config;
mod env;
mod error;
mod middleware;
mod models;
mod oauth_client;
mod routes;
mod utils;
mod ws;

pub static ALLOWED_USERS: [u64; 7] = [
    132286945031094272, // jo
    344472419085582347, // fejbijan
    245956125713760258, // fijip
    268420122090274816, // gospon menadzer
    170561008786604034, // Hetosh
    344121954124431360, // Sebek
    252114544485335051, // Metajđoš
];

pub static ALLOWED_GUILDS: [u64; 1] = [173766075484340234];


// #[cfg(debug_assertions)]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    check_required_env_variables();

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
    let users_guild_cache = Data::new(create_user_guilds_cache());
    let authorized_users_cache = Data::new(create_authorized_user_cache());

    return HttpServer::new(move || {
        // Per thread snowflake generator
        let snowflakes;
        {
            let id_arc = snowflake_thread_id.clone();
            let mut lock = id_arc.lock().unwrap();
            let epoch = UNIX_EPOCH + std::time::Duration::from_millis(1640991600000); // epoch start time 01.01.2022. 00:00
            snowflakes = Data::new(Mutex::new(SnowflakeIdGenerator::with_epoch(
                *lock, 1, epoch,
            )));
            *lock += 1;
        }

        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method()
                    .max_age(3600)
                    .send_wildcard(),
            )
            .wrap(actix_web::middleware::Logger::default())
            .app_data(oauth.clone()) // oauth2::basic::BasicClient
            .app_data(pool.clone())
            .app_data(controls_server.clone())
            .app_data(ws_channels.clone())
            .app_data(users_guild_cache.clone())
            .app_data(authorized_users_cache.clone())
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
