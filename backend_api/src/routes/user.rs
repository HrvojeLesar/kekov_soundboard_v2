use actix_web::{
    delete, get,
    web::{delete, scope, Data, Json, Path, ServiceConfig},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    error::errors::KekServerError,
    middleware::{auth_middleware::AuthService, user_guilds_middleware::UserGuildsService},
    models::{guild::Guild, ids::SoundFileId, sound_file::SoundFile, user::User},
    utils::{auth::{AuthorizedUser, AuthorizedUserExt}, cache::UserGuildsCache, make_discord_get_request, USERGUILDS},
};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/user")
            .wrap(AuthService)
            .service(get_user_files)
            .service(delete_user_file)
            .service(delete_multiple_user_files)
            .service(get_user_guilds),
    );
}

#[get("/files")]
pub async fn get_user_files(
    AuthorizedUserExt(user): AuthorizedUserExt,
    db_pool: Data<PgPool>,
) -> Result<HttpResponse, KekServerError> {
    let mut transaction = db_pool.begin().await?;
    let files =
        SoundFile::get_user_files(user.get_discord_user().get_id(), &mut transaction).await?;
    transaction.commit().await?;
    return Ok(HttpResponse::Ok().json(files));
}

#[delete("/files/{file_id}")]
pub async fn delete_user_file(
    AuthorizedUserExt(user): AuthorizedUserExt,
    db_pool: Data<PgPool>,
    file_id: Path<SoundFileId>,
) -> Result<HttpResponse, KekServerError> {
    let mut transaction = db_pool.begin().await?;

    let deleted_file = SoundFile::delete(
        &file_id.into_inner(),
        user.get_discord_user().get_id(),
        &mut transaction,
    )
    .await?;

    transaction.commit().await?;
    return Ok(HttpResponse::Ok().json(deleted_file));
}

#[derive(Serialize, Deserialize)]
pub struct FilesToDelete {
    pub files: Vec<SoundFileId>,
}

#[delete("/files")]
pub async fn delete_multiple_user_files(
    AuthorizedUserExt(user): AuthorizedUserExt,
    db_pool: Data<PgPool>,
    file_ids: Json<FilesToDelete>,
) -> Result<HttpResponse, KekServerError> {
    let mut transaction = db_pool.begin().await?;

    let deleted_files = SoundFile::delete_multiple_static(
        &file_ids.files,
        user.get_discord_user().get_id(),
        &mut transaction,
    )
    .await?;

    transaction.commit().await?;
    return Ok(HttpResponse::Ok()
        .json(serde_json::json!({ "count": deleted_files.len(), "files": deleted_files })));
}

#[get("/guilds", wrap = "UserGuildsService")]
pub async fn get_user_guilds(
    AuthorizedUserExt(authorized_user): AuthorizedUserExt,
    db_pool: Data<PgPool>,
    user_guilds_cache: Data<UserGuildsCache>,
) -> Result<HttpResponse, KekServerError> {
    let user_guilds = match user_guilds_cache.get(authorized_user.get_discord_user().get_id()) {
        Some(ug) => ug,
        None => return Err(KekServerError::UserNotInCacheError),
    };

    let mut transaction = db_pool.begin().await?;
    let guilds = Guild::get_existing_guilds(&*user_guilds, &mut transaction).await?;
    transaction.commit().await?;

    return Ok(HttpResponse::Ok().json(guilds));
}
