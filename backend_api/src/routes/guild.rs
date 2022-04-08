use actix_web::{
    delete, post,
    web::{scope, Data, Path, ServiceConfig},
    HttpResponse,
};
use sqlx::{PgPool, Postgres, Transaction};

use crate::{
    error::errors::KekServerError,
    middleware::auth_middleware::AuthService,
    models::{guild::Guild, guild_file::GuildFile, sound_file::SoundFile},
    utils::auth::AuthorizedUser,
};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/guilds")
            .wrap(AuthService)
            .service(add_sound_to_guild)
            .service(delete_sound_from_guild),
    );
}

#[post("/{guild_id}/{file_id}")]
pub async fn add_sound_to_guild(
    db_pool: Data<PgPool>,
    path: Path<(i64, i64)>,
    autorized_user: AuthorizedUser,
) -> Result<HttpResponse, KekServerError> {
    let (guild_id, file_id) = path.into_inner();
    // check if guild id is valid
    // check if file id is valid
    // check if user is allowed to add to this particular guild
    // if user is not in guild, should error

    let mut transaction = db_pool.begin().await?;

    guild_file_exist(&guild_id, &file_id, &mut transaction).await?;

    let user_guilds = autorized_user.get_guilds().await?;

    if user_guilds
        .iter()
        .find(|guild| *guild.get_id() == guild_id)
        .is_some()
    {
        GuildFile::insert_guild_file(&guild_id, &file_id, &mut transaction).await?;
    }

    transaction.commit().await?;

    return Ok(HttpResponse::Created().finish());
}

#[delete("/{guild_id}/{file_id}")]
pub async fn delete_sound_from_guild(
    db_pool: Data<PgPool>,
    path: Path<(i64, i64)>,
    autorized_user: AuthorizedUser,
) -> Result<HttpResponse, KekServerError> {
    let (guild_id, file_id) = path.into_inner();

    let mut transaction = db_pool.begin().await?;

    guild_file_exist(&guild_id, &file_id, &mut transaction).await?;

    let user_guilds = autorized_user.get_guilds().await?;

    if user_guilds
        .iter()
        .find(|guild| *guild.get_id() == guild_id)
        .is_some()
    {
        GuildFile::delete_guild_file(&guild_id, &file_id, &mut transaction).await?;
    }

    transaction.commit().await?;

    return Ok(HttpResponse::Created().finish());
}

async fn guild_file_exist(
    guild_id: &i64,
    file_id: &i64,
    transaction: &mut Transaction<'_, Postgres>,
) -> Result<(), KekServerError> {
    match Guild::get_guild_from_id(&guild_id, &mut *transaction).await? {
        Some(_) => (),
        None => return Err(KekServerError::InvalidGuildIdError),
    }

    match SoundFile::get_file_from_id(&file_id, &mut *transaction).await? {
        Some(_) => (),
        None => return Err(KekServerError::InvalidFileIdError),
    }
    return Ok(());
}
