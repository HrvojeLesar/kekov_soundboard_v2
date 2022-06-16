use actix_web::{
    delete, get,
    web::{scope, Data, Json, Path, ServiceConfig},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

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
        auth::AuthorizedUserExt,
        cache::{UserGuildsCache, UserGuildsCacheUtil},
    },
};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/user")
            .wrap(AuthService)
            .service(get_user_files)
            .service(delete_user_file)
            .service(delete_multiple_user_files)
            .service(get_user_guilds)
            .service(get_guilds_with_file)
            .service(get_enabled_user_files),
    );
}

#[get("/files")]
pub async fn get_user_files(
    AuthorizedUserExt(authorized_user): AuthorizedUserExt,
    db_pool: Data<PgPool>,
) -> Result<HttpResponse, KekServerError> {
    let mut transaction = db_pool.begin().await?;
    let files =
        SoundFile::get_user_files(&authorized_user.discord_user.id, &mut transaction).await?;
    transaction.commit().await?;
    return Ok(HttpResponse::Ok().json(files));
}

#[delete("/files/{file_id}")]
pub async fn delete_user_file(
    AuthorizedUserExt(authorized_user): AuthorizedUserExt,
    db_pool: Data<PgPool>,
    file_id: Path<SoundFileId>,
) -> Result<HttpResponse, KekServerError> {
    let mut transaction = db_pool.begin().await?;

    let deleted_file = SoundFile::delete(
        &file_id.into_inner(),
        &authorized_user.discord_user.id,
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
    AuthorizedUserExt(authorized_user): AuthorizedUserExt,
    db_pool: Data<PgPool>,
    file_ids: Json<FilesToDelete>,
) -> Result<HttpResponse, KekServerError> {
    let mut transaction = db_pool.begin().await?;

    let deleted_files = SoundFile::delete_multiple(
        &file_ids.files,
        &authorized_user.discord_user.id,
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
    let user_guilds = UserGuildsCacheUtil::get_user_guilds(&authorized_user, &user_guilds_cache)?;

    let mut transaction = db_pool.begin().await?;
    let guilds = Guild::get_existing_guilds(&*user_guilds, &mut transaction).await?;
    transaction.commit().await?;

    let guilds = guilds
        .into_iter()
        .map(|mut guild| {
            match user_guilds.iter().find(|g| &g.id == &guild.id) {
                Some(g) => {
                    guild = Guild {
                        id: g.id.clone(),
                        name: g.name.clone(),
                        icon: g.icon.clone(),
                        icon_hash: g.icon_hash.clone(),
                        time_added: guild.time_added,
                    }
                }
                None => (),
            }
            return guild;
        })
        .collect::<Vec<Guild>>();

    return Ok(HttpResponse::Ok().json(guilds));
}

#[derive(Clone, Debug, Serialize)]
struct GuildHasFile {
    guild: Guild,
    has_file: bool,
}

#[get("/guilds/{file_id}", wrap = "UserGuildsService")]
pub async fn get_guilds_with_file(
    AuthorizedUserExt(authorized_user): AuthorizedUserExt,
    db_pool: Data<PgPool>,
    user_guilds_cache: Data<UserGuildsCache>,
    file_id: Path<SoundFileId>,
) -> Result<HttpResponse, KekServerError> {
    let user_guilds = UserGuildsCacheUtil::get_user_guilds(&authorized_user, &user_guilds_cache)?;

    let mut transaction = db_pool.begin().await?;
    let guilds = Guild::get_existing_guilds(&*user_guilds, &mut transaction).await?;
    let guilds_with_file =
        GuildFile::get_matching_guilds_for_file(&guilds, &file_id, &mut transaction).await?;
    transaction.commit().await?;

    let guild_has_file_vec = guilds
        .into_iter()
        .map(|guild| {
            let has_file = guilds_with_file.get(&guild).is_some();
            GuildHasFile { guild, has_file }
        })
        .collect::<Vec<GuildHasFile>>();

    return Ok(HttpResponse::Ok().json(guild_has_file_vec));
}

#[derive(Clone, Debug, Serialize)]
struct EnabledFile {
    sound_file: SoundFile,
    enabled: bool,
}

#[get("/{guild_id}", wrap = "UserGuildsService")]
pub async fn get_enabled_user_files(
    AuthorizedUserExt(authorized_user): AuthorizedUserExt,
    db_pool: Data<PgPool>,
    user_guilds_cache: Data<UserGuildsCache>,
    guild_id: Path<GuildId>,
) -> Result<HttpResponse, KekServerError> {
    let guild_id = guild_id.into_inner();
    let user_guilds = UserGuildsCacheUtil::get_user_guilds(&authorized_user, &user_guilds_cache)?;

    if !user_guilds.iter().any(|guild| &guild.id == &guild_id) {
        return Err(KekServerError::NotInGuildError);
    }

    let mut transaction = db_pool.begin().await?;
    let files = SoundFile::get_user_files(
        &authorized_user.discord_user.id,
        &mut transaction,
    )
    .await?;
    let enabled_files = GuildFile::get_users_enabled_files_for_guild(
        &authorized_user.discord_user.id,
        &guild_id,
        &mut transaction,
    )
    .await?;
    transaction.commit().await?;

    let enabled_files = files
        .into_iter()
        .map(|f| {
            let enabled = enabled_files.get(&f).is_some();
            EnabledFile {
                sound_file: f,
                enabled,
            }
        })
        .collect::<Vec<EnabledFile>>();

    return Ok(HttpResponse::Ok().json(enabled_files));
}
