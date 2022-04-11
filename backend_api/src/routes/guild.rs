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

type GuildFileIds = Path<(i64, i64)>;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/guilds")
            .wrap(AuthService)
            .service(add_sound_to_guild)
            .service(delete_sound_from_guild),
    );
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

async fn is_user_in_guild(
    authorized_user: &AuthorizedUser,
    guild_id: &i64,
) -> Result<(), KekServerError> {
    let user_guilds = authorized_user.get_guilds().await?;

    if user_guilds
        .iter()
        .find(|guild| *guild.get_id() == *guild_id)
        .is_none()
    {
        return Err(KekServerError::NotInGuildError);
    }
    return Ok(());
}

async fn validate_query(
    authorized_user: &AuthorizedUser,
    guild_id: &i64,
    file_id: &i64,
    transaction: &mut Transaction<'_, Postgres>,
) -> Result<bool, KekServerError> {
    guild_file_exist(guild_id, file_id, &mut *transaction).await?;
    is_user_in_guild(&authorized_user, guild_id).await?;
    return Ok(true);
}

#[post("/{guild_id}/{file_id}")]
pub async fn add_sound_to_guild(
    db_pool: Data<PgPool>,
    path: GuildFileIds,
    authorized_user: AuthorizedUser,
) -> Result<HttpResponse, KekServerError> {
    let (guild_id, file_id) = path.into_inner();
    let mut transaction = db_pool.begin().await?;

    if validate_query(&authorized_user, &guild_id, &file_id, &mut transaction).await? {
        GuildFile::insert_guild_file(&guild_id, &file_id, &mut transaction).await?;
    }
    transaction.commit().await?;

    return Ok(HttpResponse::Created().finish());
}

#[delete("/{guild_id}/{file_id}")]
pub async fn delete_sound_from_guild(
    db_pool: Data<PgPool>,
    path: GuildFileIds,
    authorized_user: AuthorizedUser,
) -> Result<HttpResponse, KekServerError> {
    let (guild_id, file_id) = path.into_inner();
    let mut transaction = db_pool.begin().await?;

    if validate_query(&authorized_user, &guild_id, &file_id, &mut transaction).await? {
        GuildFile::delete_guild_file(&guild_id, &file_id, &mut transaction).await?;
    }
    transaction.commit().await?;

    return Ok(HttpResponse::Ok().finish());
}
