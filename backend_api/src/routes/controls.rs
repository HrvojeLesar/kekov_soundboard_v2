use actix::Addr;
use actix_web::{
    get,
    web::{scope, Data, Json, ServiceConfig},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    error::errors::KekServerError,
    middleware::auth_middleware::AuthService,
    models::guild_file::GuildFile,
    utils::{
        auth::AuthorizedUser,
        validation::{is_user_in_guild, validate_guild_and_file_ids},
    },
    ws::ws_server::{Controls, ControlsServer, PlayControl, ControlsServerMessage2},
};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/controls")
            .wrap(AuthService)
            .service(play_request)
            .service(stop_request),
    );
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayPayload {
    guild_id: i64,
    file_id: i64,
    channel_id: Option<i64>,
}

impl PlayPayload {
    pub fn get_guild_id(&self) -> &i64 {
        return &self.guild_id;
    }

    pub fn get_file_id(&self) -> &i64 {
        return &self.file_id;
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StopPayload {
    guild_id: i64,
}

impl StopPayload {
    pub fn get_guild_id(&self) -> &i64 {
        return &self.guild_id;
    }
}

#[get("play")]
pub async fn play_request(
    server_address: Data<Addr<ControlsServer>>,
    authorized_user: AuthorizedUser,
    req_payload: Json<PlayPayload>,
    db_pool: Data<PgPool>,
) -> Result<HttpResponse, KekServerError> {
    let mut transaction = db_pool.begin().await?;

    // TODO: Very slow (calls on discord api)
    if is_user_in_guild(&authorized_user, req_payload.get_guild_id()).await? {
        match GuildFile::get_guild_file(
            req_payload.get_guild_id(),
            req_payload.get_file_id(),
            &mut transaction,
        )
        .await?
        {
            Some(_) => {
                // let resp = server_address
                //     .send(Controls::Play(PlayControl::new(
                //         *req_payload.get_guild_id(),
                //         *req_payload.get_file_id(),
                //     )))
                //     .await?;
                let resp = server_address
                    .send(ControlsServerMessage2::new_play(*req_payload.get_guild_id(), *req_payload.get_file_id()))
                    .await?;
                transaction.commit().await?;
                return Ok(HttpResponse::Ok().finish());
            }
            None => return Err(KekServerError::GuildFileDoesNotExistError),
        }
    } else {
        return Err(KekServerError::NotInGuildError);
    }
}

#[get("stop")]
pub async fn stop_request(
    server_address: Data<Addr<ControlsServer>>,
    authorized_user: AuthorizedUser,
    req_payload: Json<StopPayload>,
    db_pool: Data<PgPool>,
) -> Result<HttpResponse, KekServerError> {
    let transaction = db_pool.begin().await?;
    let mut resp = 0;
    if is_user_in_guild(&authorized_user, req_payload.get_guild_id()).await? {
        resp = server_address.send(Controls::Stop).await?;
    }
    transaction.commit().await?;

    return Ok(HttpResponse::Ok().finish());
}
