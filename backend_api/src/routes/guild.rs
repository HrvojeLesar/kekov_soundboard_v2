use actix_web::{
    delete, get, post,
    web::{scope, Data, Json, Path, ServiceConfig},
    HttpResponse,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    error::errors::KekServerError,
    middleware::{auth_middleware::AuthService, user_guilds_middleware::UserGuildsService},
    models::{
        guild_file::GuildFile,
        ids::{GuildId, SoundFileId},
        sound_file::SoundFile,
    },
    utils::{auth::AuthorizedUserExt, cache::UserGuildsCache, validation::Validation},
};

type GuildFileIds = Path<(GuildId, SoundFileId)>;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/guilds")
            .wrap(UserGuildsService)
            .wrap(AuthService)
            .service(add_sound_to_guild)
            .service(delete_sound_from_guild)
            .service(get_guild_files)
            .service(bulk_enable),
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
    Validation::is_user_in_guild(&authorized_user, &guild_id, &user_guilds_cache)?;

    let mut transaction = db_pool.begin().await?;
    Validation::are_guild_and_file_ids_valid(
        &authorized_user.discord_user.id,
        &guild_id,
        &file_id,
        &mut transaction,
    )
    .await?;
    let guild_file = GuildFile::insert_guild_file(&guild_id, &file_id, &mut transaction).await?;
    transaction.commit().await?;
    return Ok(HttpResponse::Created().json(guild_file));
}

#[delete("/{guild_id}/{file_id}")]
pub async fn delete_sound_from_guild(
    db_pool: Data<PgPool>,
    path: GuildFileIds,
    AuthorizedUserExt(authorized_user): AuthorizedUserExt,
    user_guilds_cache: Data<UserGuildsCache>,
) -> Result<HttpResponse, KekServerError> {
    let (guild_id, file_id) = path.into_inner();
    Validation::is_user_in_guild(&authorized_user, &guild_id, &user_guilds_cache)?;

    let mut transaction = db_pool.begin().await?;
    let guild_file = GuildFile::delete_guild_file(&guild_id, &file_id, &mut transaction).await?;
    transaction.commit().await?;

    return Ok(HttpResponse::Ok().json(guild_file));
}

#[get("/{guild_id}")]
pub async fn get_guild_files(
    db_pool: Data<PgPool>,
    AuthorizedUserExt(authorized_user): AuthorizedUserExt,
    guild_id: Path<GuildId>,
    user_guilds_cache: Data<UserGuildsCache>,
) -> Result<HttpResponse, KekServerError> {
    let guild_id = guild_id.into_inner();
    Validation::is_user_in_guild(&authorized_user, &guild_id, &user_guilds_cache)?;

    let mut transaction = db_pool.begin().await?;
    let files = GuildFile::get_guild_files(&guild_id, &mut transaction).await?;
    transaction.commit().await?;

    return Ok(HttpResponse::Ok().json(files));
}

#[derive(Deserialize)]
pub struct Bulk {
    guilds: Vec<GuildId>,
    files: Vec<SoundFileId>,
}

#[post("/bulkenable")]
pub async fn bulk_enable(
    db_pool: Data<PgPool>,
    AuthorizedUserExt(authorized_user): AuthorizedUserExt,
    bulk_payload: Json<Bulk>,
    user_guilds_cache: Data<UserGuildsCache>,
) -> Result<HttpResponse, KekServerError> {
    Validation::is_user_in_provided_guilds(
        &authorized_user,
        &bulk_payload.guilds,
        &user_guilds_cache,
    )?;
    let mut transaction = db_pool.begin().await?;
    {
        let files =
            SoundFile::get_user_files(&authorized_user.discord_user.id, &mut transaction).await?;
        Validation::user_owns_provided_files(&bulk_payload.files, &files)?;
    }

    GuildFile::bulk_insert(&bulk_payload.guilds, &bulk_payload.files, &mut transaction).await?;
    transaction.commit().await?;

    return Ok(HttpResponse::Created().finish());
}
