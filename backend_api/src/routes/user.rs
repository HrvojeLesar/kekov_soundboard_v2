use actix_web::{
    delete, get,
    web::{delete, scope, Data, Json, Path, ServiceConfig},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    error::errors::KekServerError,
    middleware::auth_middleware::AuthService,
    models::{guild::Guild, sound_file::SoundFile, user::User},
    utils::{auth::AuthorizedUser, make_discord_get_request},
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
    user: AuthorizedUser,
    db_pool: Data<PgPool>,
) -> Result<HttpResponse, KekServerError> {
    let mut transaction = db_pool.begin().await?;

    let files = sqlx::query_as!(
        SoundFile,
        "
        SELECT * FROM files
        WHERE owner = $1
        ",
        *user.get_discord_user().get_id()
    )
    .fetch_all(&mut transaction)
    .await?;

    transaction.commit().await?;
    return Ok(HttpResponse::Ok().json(files));
}

#[delete("/files/{file_id}")]
pub async fn delete_user_file(
    user: AuthorizedUser,
    db_pool: Data<PgPool>,
    file_id: Path<i64>,
) -> Result<HttpResponse, KekServerError> {
    let mut transaction = db_pool.begin().await?;

    let deleted_file = SoundFile::delete_static(
        file_id.into_inner(),
        *user.get_discord_user().get_id(),
        &mut transaction,
    )
    .await?;

    transaction.commit().await?;
    return Ok(HttpResponse::Ok().json(deleted_file));
}

#[derive(Serialize, Deserialize)]
pub struct FilesToDelete {
    pub files: Vec<i64>,
}

#[delete("/files")]
pub async fn delete_multiple_user_files(
    user: AuthorizedUser,
    db_pool: Data<PgPool>,
    file_ids: Json<FilesToDelete>,
) -> Result<HttpResponse, KekServerError> {
    let mut transaction = db_pool.begin().await?;

    let deleted_files = SoundFile::delete_multiple_static(
        &file_ids.files,
        *user.get_discord_user().get_id(),
        &mut transaction,
    )
    .await?;

    transaction.commit().await?;
    return Ok(HttpResponse::Ok()
        .json(serde_json::json!({ "count": deleted_files.len(), "files": deleted_files })));
}

#[get("/guilds")]
pub async fn get_user_guilds(
    user: AuthorizedUser,
    db_pool: Data<PgPool>,
) -> Result<HttpResponse, KekServerError> {
    // get users guilds
    // return matching active guilds from db
    let mut transaction = db_pool.begin().await?;

    let user_guilds = make_discord_get_request(user, "/users/@me/guilds")
        .await?
        .json()
        .await?;
    let guilds = Guild::get_existing_guilds(&user_guilds, &mut transaction).await?;

    transaction.commit().await?;
    return Ok(HttpResponse::Ok().json(guilds));
}
