use std::time::{Duration, Instant};

use actix::{clock::timeout, Addr};
use actix_web::{
    get, post,
    web::{scope, Data, Json, ServiceConfig},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio::sync::oneshot::channel;

use crate::{
    error::errors::KekServerError,
    middleware::auth_middleware::AuthService,
    models::{
        guild_file::GuildFile,
        ids::{ChannelId, GuildId, SoundFileId},
    },
    utils::{
        auth::AuthorizedUser,
        validation::{is_user_in_guild, validate_guild_and_file_ids},
    },
    ws::{
        ws_server::{Controls, ControlsServer, ControlsServerMessage2, PlayControl},
        ws_session::WsSessionCommChannels,
    },
};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/controls")
            .wrap(AuthService)
            .service(test_control)
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

#[post("play")]
pub async fn play_request(
    server_address: Data<Addr<ControlsServer>>,
    authorized_user: AuthorizedUser,
    req_payload: Json<PlayPayload>,
    db_pool: Data<PgPool>,
    ws_channels: Data<WsSessionCommChannels>,
) -> Result<HttpResponse, KekServerError> {
    let mut transaction = db_pool.begin().await?;

    // TODO: Very slow (calls on discord api)
    if is_user_in_guild(&authorized_user, &req_payload.guild_id).await? {
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
                let control =
                    ControlsServerMessage2::new_play(payload.guild_id, payload.file_id);
                let id = control.get_id();

                let (sender, receiver) = channel();
                {
                    let mut lock = ws_channels.write().await;
                    lock.insert(id, sender);
                }
                server_address.send(control).await?;
                let resp = timeout(Duration::from_secs(10), receiver).await???;
                return Ok(HttpResponse::Ok().finish());
            }
            None => return Err(KekServerError::GuildFileDoesNotExistError),
        }
    } else {
        return Err(KekServerError::NotInGuildError);
    }
}

#[post("stop")]
pub async fn stop_request(
    server_address: Data<Addr<ControlsServer>>,
    authorized_user: AuthorizedUser,
    req_payload: Json<StopPayload>,
    db_pool: Data<PgPool>,
    ws_channels: Data<WsSessionCommChannels>,
) -> Result<HttpResponse, KekServerError> {
    let transaction = db_pool.begin().await?;
    let is_user_in_guild = is_user_in_guild(&authorized_user, &req_payload.guild_id).await?;
    transaction.commit().await?;

    if is_user_in_guild {
        let control = ControlsServerMessage2::new_stop();
        let id = control.get_id();

        let (sender, receiver) = channel();
        {
            let mut lock = ws_channels.write().await;
            lock.insert(id, sender);
        }

        server_address.send(control).await?;
        let resp = timeout(Duration::from_secs(10), receiver).await???;
    }

    return Ok(HttpResponse::Ok().finish());
}

// Possile implementation over channels instead of actors
#[post("testcontrol")]
pub async fn test_control(
    server_address: Data<Addr<ControlsServer>>,
    ws_channels: Data<WsSessionCommChannels>,
    req_payload: Json<PlayPayload>,
) -> Result<HttpResponse, KekServerError> {
    let payload = req_payload.into_inner();
    let control = ControlsServerMessage2::new_play(payload.guild_id, payload.file_id);
    let id = control.get_id();

    let (sender, receiver) = channel();
    {
        let mut t = ws_channels.write().await;
        t.insert(id, sender);
    }

    server_address.send(control).await?;
    let resp = timeout(Duration::from_secs(10), receiver).await???;
    return Ok(HttpResponse::Ok().finish());
}
