use std::time::Duration;

use actix::{clock::timeout, Addr};
use actix_web::{
    post,
    web::{scope, Data, Json, ServiceConfig},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio::sync::oneshot::{channel, Receiver};

use crate::{
    error::errors::KekServerError,
    middleware::{auth_middleware::AuthService, user_guilds_middleware::UserGuildsService},
    models::{
        guild_file::GuildFile,
        ids::{ChannelId, GuildId, SoundFileId},
        sound_file::SoundFile,
    },
    utils::{
        auth::AuthorizedUserExt,
        cache::{UserGuildsCache, UserGuildsCacheUtil},
    },
    ws::{
        ws_server::{ControlsServer, ControlsServerMessage},
        ws_session::WsSessionCommChannels,
    },
};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/controls")
            .wrap(UserGuildsService)
            .wrap(AuthService)
            .service(play_request)
            .service(stop_request)
            .service(skip_request)
            .service(queue_request),
    );
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayPayload {
    pub guild_id: GuildId,
    pub file_id: SoundFileId,
    pub channel_id: Option<ChannelId>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StopPayload {
    pub guild_id: GuildId,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SkipPayload {
    pub guild_id: GuildId,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueuePayload {
    pub guild_id: GuildId,
}

async fn cleanup(id: &u128, ws_channels: &Data<WsSessionCommChannels>) {
    {
        let mut lock = ws_channels.write().await;
        lock.remove(&id);
    }
}
async fn create_channels(
    id: u128,
    ws_channels: &Data<WsSessionCommChannels>,
) -> Receiver<Result<ControlsServerMessage, ControlsServerMessage>> {
    let (sender, receiver) = channel();
    {
        let mut lock = ws_channels.write().await;
        lock.insert(id, sender);
    }
    return receiver;
}

async fn wait_for_ws_response(
    id: &u128,
    receiver: Receiver<Result<ControlsServerMessage, ControlsServerMessage>>,
    ws_channels: Data<WsSessionCommChannels>,
) -> Result<ControlsServerMessage, KekServerError> {
    return match timeout(Duration::from_secs(10), receiver).await?? {
        Ok(o) => Ok(o),
        Err(e) => {
            cleanup(id, &ws_channels).await;
            return Ok(e);
        }
    };
}

async fn send_command(
    control: ControlsServerMessage,
    server_address: Data<Addr<ControlsServer>>,
    ws_channels: Data<WsSessionCommChannels>,
) -> Result<ControlsServerMessage, KekServerError> {
    let id = control.get_id();

    let receiver = create_channels(id, &ws_channels).await;
    match server_address.send(control).await {
        Ok(_) => (),
        Err(e) => {
            cleanup(&id, &ws_channels).await;
            return Err(e.into());
        }
    }
    return wait_for_ws_response(&id, receiver, ws_channels).await;
}

#[post("play")]
pub async fn play_request(
    server_address: Data<Addr<ControlsServer>>,
    AuthorizedUserExt(authorized_user): AuthorizedUserExt,
    req_payload: Json<PlayPayload>,
    db_pool: Data<PgPool>,
    ws_channels: Data<WsSessionCommChannels>,
    user_guilds_cache: Data<UserGuildsCache>,
) -> Result<HttpResponse, KekServerError> {
    let user_guilds = UserGuildsCacheUtil::get_user_guilds(&authorized_user, &user_guilds_cache)?;

    if !user_guilds
        .iter()
        .any(|guild| guild.get_id() == &req_payload.guild_id)
    {
        return Err(KekServerError::NotInGuildError);
    }

    let mut transaction = db_pool.begin().await?;

    match GuildFile::get_guild_file(
        &req_payload.guild_id,
        &req_payload.file_id,
        &mut transaction,
    )
    .await?
    {
        Some(guild_file) => {
            transaction.commit().await?;

            let payload = req_payload.into_inner();
            let control = ControlsServerMessage::new_play(guild_file, None);
            let resp = send_command(control, server_address, ws_channels).await?;

            return Ok(HttpResponse::Ok().json(resp));
        }
        None => return Err(KekServerError::GuildFileDoesNotExistError),
    }
}

#[post("stop")]
pub async fn stop_request(
    server_address: Data<Addr<ControlsServer>>,
    AuthorizedUserExt(authorized_user): AuthorizedUserExt,
    Json(stop_payload): Json<StopPayload>,
    ws_channels: Data<WsSessionCommChannels>,
    user_guilds_cache: Data<UserGuildsCache>,
) -> Result<HttpResponse, KekServerError> {
    let user_guilds = UserGuildsCacheUtil::get_user_guilds(&authorized_user, &user_guilds_cache)?;

    if !user_guilds
        .iter()
        .any(|guild| guild.get_id() == &stop_payload.guild_id)
    {
        return Err(KekServerError::NotInGuildError);
    }

    let control = ControlsServerMessage::new_stop(stop_payload.guild_id);
    let resp = send_command(control, server_address, ws_channels).await?;

    return Ok(HttpResponse::Ok().json(resp));
}

#[post("skip")]
pub async fn skip_request(
    server_address: Data<Addr<ControlsServer>>,
    AuthorizedUserExt(authorized_user): AuthorizedUserExt,
    Json(skip_payload): Json<SkipPayload>,
    ws_channels: Data<WsSessionCommChannels>,
    user_guilds_cache: Data<UserGuildsCache>,
) -> Result<HttpResponse, KekServerError> {
    let user_guilds = UserGuildsCacheUtil::get_user_guilds(&authorized_user, &user_guilds_cache)?;

    if !user_guilds
        .iter()
        .any(|guild| guild.get_id() == &skip_payload.guild_id)
    {
        return Err(KekServerError::NotInGuildError);
    }

    let control = ControlsServerMessage::new_skip(skip_payload.guild_id);
    let resp = send_command(control, server_address, ws_channels).await?;

    return Ok(HttpResponse::Ok().json(resp));
}

#[post("queue")]
pub async fn queue_request(
    server_address: Data<Addr<ControlsServer>>,
    AuthorizedUserExt(authorized_user): AuthorizedUserExt,
    Json(queue_payload): Json<QueuePayload>,
    ws_channels: Data<WsSessionCommChannels>,
    user_guilds_cache: Data<UserGuildsCache>,
) -> Result<HttpResponse, KekServerError> {
    let user_guilds = UserGuildsCacheUtil::get_user_guilds(&authorized_user, &user_guilds_cache)?;

    if !user_guilds
        .iter()
        .any(|guild| guild.get_id() == &queue_payload.guild_id)
    {
        return Err(KekServerError::NotInGuildError);
    }

    let control = ControlsServerMessage::new_queue(queue_payload.guild_id);
    let resp = send_command(control, server_address, ws_channels).await?;
    match resp.queue {
        Some(q) => return Ok(HttpResponse::Ok().json(q)),
        None => return Ok(HttpResponse::Ok().json(Vec::<SoundFile>::new())),
    }
}
