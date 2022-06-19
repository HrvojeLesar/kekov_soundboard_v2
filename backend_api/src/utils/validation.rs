use actix_web::web::Data;
use sqlx::{Postgres, Transaction};

use crate::{
    error::errors::KekServerError,
    models::{
        guild::Guild,
        ids::{GuildId, SoundFileId, UserId},
        sound_file::SoundFile,
    },
};

use super::{auth::AuthorizedUser, cache::UserGuildsCache};

pub struct Validation;

impl Validation {
    pub async fn is_user_in_guild(
        authorized_user: &AuthorizedUser,
        guild_id: &GuildId,
        user_guilds_cache: &Data<UserGuildsCache>,
    ) -> Result<(), KekServerError> {
        match user_guilds_cache.get(&authorized_user.discord_user.id) {
            Some(guilds) => {
                guilds
                    .iter()
                    .find(|guild| &guild.id == guild_id)
                    .ok_or(KekServerError::NotInGuildError)?;
            }
            None => return Err(KekServerError::UserNotInCacheError),
        }

        return Ok(());
    }

    pub async fn is_user_in_provided_guilds(
        authorized_user: &AuthorizedUser,
        guild_ids: &[GuildId],
        user_guilds_cache: &Data<UserGuildsCache>,
    ) -> Result<(), KekServerError> {
        match user_guilds_cache.get(&authorized_user.discord_user.id) {
            Some(guilds) => {
                for id in guild_ids {
                    if !guilds.iter().any(|guild| &guild.id == id) {
                        return Err(KekServerError::NotInGuildError);
                    }
                }
            }
            None => return Err(KekServerError::UserNotInCacheError),
        }
        return Ok(());
    }

    pub async fn user_owns_provided_files(
        file_ids: &[SoundFileId],
        user_owned_files: &[SoundFile],
    ) -> Result<(), KekServerError> {
        for id in file_ids {
            if !user_owned_files.iter().any(|s| &s.id == id) {
                return Err(KekServerError::UnauthorizedFileAccessError(format!(
                    "User doesn't own file with id: [{}]",
                    id.0
                )));
            }
        }
        return Ok(());
    }

    pub async fn are_guild_and_file_ids_valid(
        user_id: &UserId,
        guild_id: &GuildId,
        file_id: &SoundFileId,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<(), KekServerError> {
        match Guild::get_guild_from_id(guild_id, &mut *transaction).await? {
            Some(_) => (),
            None => return Err(KekServerError::InvalidGuildIdError),
        }

        match SoundFile::get_file(file_id, user_id, &mut *transaction).await? {
            Some(_) => (),
            None => return Err(KekServerError::InvalidFileIdError),
        }
        return Ok(());
    }
}
