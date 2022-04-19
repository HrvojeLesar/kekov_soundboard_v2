use actix_web::{
    delete, get, post,
    web::{scope, Data, Path, ServiceConfig},
    HttpResponse,
};
use sqlx::{PgPool, Postgres, Transaction};

use crate::{
    error::errors::KekServerError,
    middleware::auth_middleware::AuthService,
    models::{guild::Guild, guild_file::GuildFile, sound_file::SoundFile},
    utils::{auth::AuthorizedUser, validation::{validate_guild_and_file_ids, is_user_in_guild}},
};

type GuildFileIds = Path<(i64, i64)>;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/guilds")
            .wrap(AuthService)
            .service(add_sound_to_guild)
            .service(delete_sound_from_guild)
            .service(get_guild_files),
    );
}

#[post("/{guild_id}/{file_id}")]
pub async fn add_sound_to_guild(
    db_pool: Data<PgPool>,
    path: GuildFileIds,
    authorized_user: AuthorizedUser,
) -> Result<HttpResponse, KekServerError> {
    let (guild_id, file_id) = path.into_inner();
    let mut transaction = db_pool.begin().await?;

    if validate_guild_and_file_ids(&authorized_user, &guild_id, &file_id, &mut transaction).await? {
        GuildFile::insert_guild_file(&guild_id, &file_id, &mut transaction).await?;
        transaction.commit().await?;
        return Ok(HttpResponse::Created().finish());
    } else {
        return Err(KekServerError::NotInGuildError);
    }

}

#[delete("/{guild_id}/{file_id}")]
pub async fn delete_sound_from_guild(
    db_pool: Data<PgPool>,
    path: GuildFileIds,
    authorized_user: AuthorizedUser,
) -> Result<HttpResponse, KekServerError> {
    let (guild_id, file_id) = path.into_inner();
    let mut transaction = db_pool.begin().await?;

    if validate_guild_and_file_ids(&authorized_user, &guild_id, &file_id, &mut transaction).await? {
        GuildFile::delete_guild_file(&guild_id, &file_id, &mut transaction).await?;
        transaction.commit().await?;
        return Ok(HttpResponse::Ok().finish());
    } else {
        return Err(KekServerError::NotInGuildError);
    }

}

#[get("/{guild_id}")]
pub async fn get_guild_files(
    db_pool: Data<PgPool>,
    authorized_user: AuthorizedUser,
    guild_id: Path<i64>,
) -> Result<HttpResponse, KekServerError> {
    let guild_id = guild_id.into_inner();
    if is_user_in_guild(&authorized_user, &guild_id).await? {
        let mut transaction = db_pool.begin().await?;
        let files = GuildFile::get_guild_files(&guild_id, &mut transaction).await?;
        transaction.commit().await?;
        return Ok(HttpResponse::Ok().json(files));
    }
    return Ok(HttpResponse::Ok().json("[]"));
}
