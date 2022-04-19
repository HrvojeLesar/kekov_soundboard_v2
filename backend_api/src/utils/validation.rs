use sqlx::{Transaction, Postgres};

use crate::{error::errors::KekServerError, models::{guild::Guild, sound_file::SoundFile}};

use super::auth::AuthorizedUser;

pub async fn guild_and_file_exist(
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

pub async fn is_user_in_guild(
    authorized_user: &AuthorizedUser,
    guild_id: &i64,
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
    guild_id: &i64,
    file_id: &i64,
    transaction: &mut Transaction<'_, Postgres>,
) -> Result<bool, KekServerError> {
    guild_and_file_exist(guild_id, file_id, &mut *transaction).await?;
    return Ok(is_user_in_guild(&authorized_user, guild_id).await?);
}
