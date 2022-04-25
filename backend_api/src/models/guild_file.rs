use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};

use crate::error::errors::KekServerError;

use super::{
    ids::{GuildId, SoundFileId, UserId},
    sound_file::SoundFile,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuildFile {
    guild_id: GuildId,
    file_id: SoundFileId,
}

impl GuildFile {
    pub async fn insert_guild_file(
        guild_id: &GuildId,
        file_id: &SoundFileId,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<(), KekServerError> {
        sqlx::query!(
            "
            INSERT INTO guild_file (guild_id, file_id)
            VALUES ($1, $2)
            ",
            guild_id.0 as i64,
            file_id.0 as i64
        )
        .execute(&mut *transaction)
        .await?;
        return Ok(());
    }

    pub async fn delete_guild_file(
        guild_id: &GuildId,
        file_id: &SoundFileId,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<(), KekServerError> {
        sqlx::query!(
            "
            DELETE FROM guild_file
            WHERE guild_id = $1 AND file_id = $2
            ",
            guild_id.0 as i64,
            file_id.0 as i64
        )
        .execute(&mut *transaction)
        .await?;
        return Ok(());
    }

    pub async fn get_guild_files(
        guild_id: &GuildId,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Vec<SoundFile>, KekServerError> {
        let records = sqlx::query!(
            "
            SELECT files.* FROM files
            INNER JOIN guild_file ON guild_file.guild_id = $1
            AND files.id = guild_file.file_id
            ",
            guild_id.0 as i64,
        )
        .fetch_all(&mut *transaction)
        .await?;

        let guild_files = records
            .into_iter()
            .map(|r| {
                let owner = match r.owner {
                    Some(o) => Some(UserId(o as u64)),
                    None => None,
                };
                SoundFile {
                    id: SoundFileId(r.id as u64),
                    owner,
                    display_name: r.display_name,
                }
            })
            .collect::<Vec<SoundFile>>();

        return Ok(guild_files);
    }

    pub async fn get_guild_file(
        guild_id: &GuildId,
        file_id: &SoundFileId,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<Self>, KekServerError> {
        match sqlx::query!(
            "
            SELECT * FROM guild_file
            WHERE guild_id = $1 AND file_id = $2
            ",
            guild_id.0 as i64,
            file_id.0 as i64
        )
        .fetch_optional(&mut *transaction)
        .await?
        {
            Some(guild_file) => {
                return Ok(Some(Self {
                    guild_id: GuildId(guild_file.guild_id as u64),
                    file_id: SoundFileId(guild_file.file_id as u64),
                }))
            }
            None => return Ok(None),
        }
    }
}
