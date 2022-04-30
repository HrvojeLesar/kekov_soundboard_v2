use std::time::{Duration, Instant};

use actix::{clock::timeout, Addr};
use actix_web::{
    dev::Service,
    get, post,
    web::{scope, Data, Json, ServiceConfig},
    HttpResponse,
};
use log::warn;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio::sync::oneshot::{channel, Receiver};

use crate::{
    error::errors::KekServerError,
    middleware::{auth_middleware::AuthService, user_guilds_middleware::UserGuildsService},
    models::{
        guild_file::GuildFile,
        ids::{ChannelId, GuildId, Id, SoundFileId},
    },
    utils::{
        auth::{AuthorizedUser, AuthorizedUserExt},
        cache::UserGuildsCache,
        validation::{is_user_in_guild, validate_guild_and_file_ids},
    },
    ws::{
        ws_server::{ClientError, ControlsServer, ControlsServerMessage, PlayControl},
        ws_session::WsSessionCommChannels,
    },
};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/controls")
            .wrap(UserGuildsService)
            .wrap(AuthService)
            .service(play_request)
            .service(stop_request),
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

async fn create_channels(
    id: u128,
    ws_channels: &Data<WsSessionCommChannels>,
) -> Receiver<Result<(), ClientError>> {
    let (sender, receiver) = channel();
    {
        let mut lock = ws_channels.write().await;
        lock.insert(id, sender);
    }
    return receiver;
}

async fn wait_for_ws_response(
    id: &u128,
    receiver: Receiver<Result<(), ClientError>>,
    ws_channels: Data<WsSessionCommChannels>,
) -> Result<(), KekServerError> {
    return match timeout(Duration::from_secs(10), receiver).await?? {
        Ok(o) => Ok(o),
        Err(e) => {
            {
                let mut lock = ws_channels.write().await;
                lock.remove(id);
            }
            return Err(e.into());
        }
    };
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
    let user_guilds = match user_guilds_cache.get(authorized_user.get_discord_user().get_id()) {
        Some(ug) => ug,
        None => return Err(KekServerError::UserNotInCacheError),
    };

    if !user_guilds.contains(&req_payload.guild_id) {
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
        Some(_) => {
            transaction.commit().await?;

            let payload = req_payload.into_inner();
            let control = ControlsServerMessage::new_play(payload.guild_id, payload.file_id);
            let id = control.get_id();

            let receiver = create_channels(id, &ws_channels).await;
            server_address.send(control).await?;
            let resp = wait_for_ws_response(&id, receiver, ws_channels).await?;

            return Ok(HttpResponse::Ok().finish());
        }
        None => return Err(KekServerError::GuildFileDoesNotExistError),
    }
}

#[post("stop")]
pub async fn stop_request(
    server_address: Data<Addr<ControlsServer>>,
    AuthorizedUserExt(authorized_user): AuthorizedUserExt,
    req_payload: Json<StopPayload>,
    db_pool: Data<PgPool>,
    ws_channels: Data<WsSessionCommChannels>,
    user_guilds_cache: Data<UserGuildsCache>,
) -> Result<HttpResponse, KekServerError> {
    let user_guilds = match user_guilds_cache.get(authorized_user.get_discord_user().get_id()) {
        Some(ug) => ug,
        None => return Err(KekServerError::UserNotInCacheError),
    };

    if !user_guilds.contains(&req_payload.guild_id) {
        return Err(KekServerError::NotInGuildError);
    }

    let control = ControlsServerMessage::new_stop();
    let id = control.get_id();

    let receiver = create_channels(id, &ws_channels).await;
    server_address.send(control).await?;
    let resp = wait_for_ws_response(&id, receiver, ws_channels).await?;

    return Ok(HttpResponse::Ok().finish());
}
