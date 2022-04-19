use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};

use crate::error::errors::KekServerError;

use super::sound_file::SoundFile;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuildFile {
    guild_id: i64,
    file_id: i64,
}

impl GuildFile {
    pub async fn insert_guild_file(
        guild_id: &i64,
        file_id: &i64,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<(), KekServerError> {
        sqlx::query!(
            "
            INSERT INTO guild_file (guild_id, file_id)
            VALUES ($1, $2)
            ",
            guild_id,
            file_id
        )
        .execute(&mut *transaction)
        .await?;
        return Ok(());
    }

    pub async fn delete_guild_file(
        guild_id: &i64,
        file_id: &i64,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<(), KekServerError> {
        sqlx::query!(
            "
            DELETE FROM guild_file
            WHERE guild_id = $1 AND file_id = $2
            ",
            guild_id,
            file_id
        )
        .execute(&mut *transaction)
        .await?;
        return Ok(());
    }

    pub async fn get_guild_files(
        guild_id: &i64,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Vec<SoundFile>, KekServerError> {
        return Ok(sqlx::query_as!(
            SoundFile,
            "
            SELECT files.* FROM files
            INNER JOIN guild_file ON guild_file.guild_id = $1
            AND files.id = guild_file.file_id
            ",
            guild_id,
        )
        .fetch_all(&mut *transaction)
        .await?);
    }

    pub async fn get_guild_file(
        guild_id: &i64,
        file_id: &i64,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<Self>, KekServerError> {
        return Ok(sqlx::query_as!(
            Self,
            "
            SELECT * FROM guild_file
            WHERE guild_id = $1 AND file_id = $2
            ",
            guild_id,
            file_id
        )
        .fetch_optional(&mut *transaction)
        .await?);
    }
}
