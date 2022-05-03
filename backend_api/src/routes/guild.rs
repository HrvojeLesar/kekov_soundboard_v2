use actix_web::{
    delete, get, post,
    web::{scope, Data, Path, ServiceConfig},
    HttpResponse,
};
use sqlx::{PgPool, Postgres, Transaction};

use crate::{
    error::errors::KekServerError,
    middleware::{auth_middleware::AuthService, user_guilds_middleware::UserGuildsService},
    models::{
        guild::Guild,
        guild_file::GuildFile,
        ids::{GuildId, SoundFileId},
        sound_file::SoundFile,
    },
    utils::{
        auth::{AuthorizedUser, AuthorizedUserExt},
        cache::UserGuildsCache,
        validation::{
            guild_and_file_exist, is_user_in_guild, validate_guild_and_file_ids, Validation,
        },
    },
};

type GuildFileIds = Path<(GuildId, SoundFileId)>;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/guilds")
            .wrap(UserGuildsService)
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
    AuthorizedUserExt(authorized_user): AuthorizedUserExt,
    user_guilds_cache: Data<UserGuildsCache>,
) -> Result<HttpResponse, KekServerError> {
    let (guild_id, file_id) = path.into_inner();
    Validation::is_user_in_guild(&authorized_user, &guild_id, user_guilds_cache).await?;

    let mut transaction = db_pool.begin().await?;
    guild_and_file_exist(&guild_id, &file_id, &mut transaction).await?;
    GuildFile::insert_guild_file(&guild_id, &file_id, &mut transaction).await?;
    transaction.commit().await?;
    return Ok(HttpResponse::Created().finish());
}

#[delete("/{guild_id}/{file_id}")]
pub async fn delete_sound_from_guild(
    db_pool: Data<PgPool>,
    path: GuildFileIds,
    AuthorizedUserExt(authorized_user): AuthorizedUserExt,
    user_guilds_cache: Data<UserGuildsCache>,
) -> Result<HttpResponse, KekServerError> {
    let (guild_id, file_id) = path.into_inner();
    Validation::is_user_in_guild(&authorized_user, &guild_id, user_guilds_cache).await?;

    let mut transaction = db_pool.begin().await?;
    guild_and_file_exist(&guild_id, &file_id, &mut transaction).await?;
    GuildFile::delete_guild_file(&guild_id, &file_id, &mut transaction).await?;
    transaction.commit().await?;

    return Ok(HttpResponse::Ok().finish());
}

#[get("/{guild_id}")]
pub async fn get_guild_files(
    db_pool: Data<PgPool>,
    AuthorizedUserExt(authorized_user): AuthorizedUserExt,
    guild_id: Path<GuildId>,
    user_guilds_cache: Data<UserGuildsCache>,
) -> Result<HttpResponse, KekServerError> {
    let guild_id = guild_id.into_inner();
    Validation::is_user_in_guild(&authorized_user, &guild_id, user_guilds_cache).await?;

    let mut transaction = db_pool.begin().await?;
    let files = GuildFile::get_guild_files(&guild_id, &mut transaction).await?;
    transaction.commit().await?;

    return Ok(HttpResponse::Ok().json(files));
}
