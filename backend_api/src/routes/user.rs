use actix_web::{
    delete, get,
    web::{delete, scope, Data, Json, Path, ServiceConfig},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, PgPool};

use crate::{
    error::errors::KekServerError, middleware::auth_middleware::AuthService,
    models::sound_file::SoundFile, utils::auth::AuthorizedUser,
};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/user")
            .wrap(AuthService)
            .service(get_user_files)
            .service(delete_user_file)
            .service(delete_multiple_user_files),
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
