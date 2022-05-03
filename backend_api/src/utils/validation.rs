use actix_web::web::Data;
use sqlx::{Postgres, Transaction};

use crate::{
    error::errors::KekServerError,
    models::{
        guild::Guild,
        ids::{GuildId, SoundFileId},
        sound_file::SoundFile,
    },
};

use super::{auth::AuthorizedUser, cache::UserGuildsCache};

pub struct Validation;

impl Validation {
    pub async fn is_user_in_guild(
        authorized_user: &AuthorizedUser,
        guild_id: &GuildId,
        user_guilds_cache: Data<UserGuildsCache>,
    ) -> Result<(), KekServerError> {
        match user_guilds_cache.get(authorized_user.get_discord_user().get_id()) {
            Some(guilds) => {
                guilds
                    .iter()
                    .find(|g_id| **g_id == *guild_id)
                    .ok_or(KekServerError::NotInGuildError)?;
            }
            None => return Err(KekServerError::UserNotInCacheError),
        }

        return Ok(());
    }
}

pub async fn guild_and_file_exist(
    guild_id: &GuildId,
    file_id: &SoundFileId,
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

pub async fn is_user_in_guild(
    authorized_user: &AuthorizedUser,
    guild_id: &GuildId,
) -> Result<bool, KekServerError> {
    let user_guilds = authorized_user.get_guilds().await?;

    if user_guilds
        .iter()
        .find(|guild| *guild.get_id() == *guild_id)
        .is_none()
    {
        return Ok(false);
    }
    return Ok(true);
}

pub async fn validate_guild_and_file_ids(
    authorized_user: &AuthorizedUser,
    guild_id: &GuildId,
    file_id: &SoundFileId,
    transaction: &mut Transaction<'_, Postgres>,
) -> Result<bool, KekServerError> {
    guild_and_file_exist(guild_id, file_id, &mut *transaction).await?;
    return Ok(is_user_in_guild(&authorized_user, guild_id).await?);
}
