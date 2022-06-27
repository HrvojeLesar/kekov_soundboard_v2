use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::UNIX_EPOCH,
};

use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use env::check_required_env_variables;
use log::warn;
use routes::{not_found::not_found, routes_config, status::Status};

use dotenv::dotenv;
use snowflake::SnowflakeIdGenerator;
use tokio::sync::RwLock;
use utils::cache::{
    create_auth_middlware_queue_cache, create_authorized_user_cache, create_user_guilds_cache,
    create_user_guilds_middlware_queue_cache,
};
use ws::{
    channels_server::ChannelsServer,
    ws_server::{self, ControlsServer},
    ws_session::WsSessionCommChannels,
};

mod database;
mod discord_client_config;
mod env;
mod error;
mod middleware;
mod models;
mod oauth_client;
mod routes;
mod scheduler;
mod utils;
mod ws;

// WARN: HARDCODED LIMITS
pub static ALLOWED_USERS: [u64; 7] = [
    132286945031094272, // jo
    344472419085582347, // fejbijan
    245956125713760258, // fijip
    268420122090274816, // gospon menadzer
    170561008786604034, // Hetosh
    344121954124431360, // Sebek
    252114544485335051, // Metajđoš
];

// #[cfg(debug_assertions)]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    check_required_env_variables();

    let bind_address = format!(
        "localhost:{}",
        dotenv::var("PORT").unwrap_or_else(|_| "8080".to_string())
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
    let status = Data::new(RwLock::new(Status::new()));
    let user_guilds_middleware_queue =
        Data::new(Mutex::new(create_user_guilds_middlware_queue_cache()));
    let auth_middleware_queue = Data::new(Mutex::new(create_auth_middlware_queue_cache()));

    let channels_server = Data::new(ChannelsServer::new(
        authorized_users_cache.clone(),
        auth_middleware_queue.clone(),
        users_guild_cache.clone(),
        user_guilds_middleware_queue.clone(),
    ));

    let mut scheduler = scheduler::Scheduler::new();

    let status_ref = status.clone();
    let ws_channels_ref = ws_channels.clone();
    let controls_server_ref = controls_server.clone();
    let ugmq_ref = user_guilds_middleware_queue.clone();
    let amq_ref = auth_middleware_queue.clone();
    scheduler.run(std::time::Duration::from_secs(1), move || {
        let ws_channels_ref = ws_channels_ref.clone();
        let status_ref = status_ref.clone();
        let controls_server_ref = controls_server_ref.clone();
        let ugmq_ref = ugmq_ref.clone();
        let amq_ref = amq_ref.clone();
        async move {
            let mut status = status_ref.write().await;
            status.ws_channel_num = ws_channels_ref.read().await.len();
            match controls_server_ref.send(ws_server::Status {}).await {
                Ok(n) => status.ws_clients_num = n,
                Err(e) => {
                    warn!(
                        "Failed to fetch control server websocket status! Error: {}",
                        e
                    );
                }
            }
            {
                status.auth_queue_cache = ugmq_ref.lock().unwrap().0.entry_count() as usize;
                status.guilds_queue_cache = amq_ref.lock().unwrap().0.entry_count() as usize;
            }
        }
    });

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
            .app_data(status.clone())
            .app_data(user_guilds_middleware_queue.clone())
            .app_data(auth_middleware_queue.clone())
            .app_data(snowflakes)
            .app_data(channels_server.clone())
            .configure(routes_config)
            .default_service(actix_web::web::to(not_found))
    })
    // .bind_rustls(&bind_address, config)?
    .bind(bind_address)?
    // .workers(1)
    .run()
    .await;
}
