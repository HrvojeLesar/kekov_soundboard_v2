use std::collections::HashSet;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};

use crate::error::errors::KekServerError;

use super::{
    guild::Guild,
    ids::{GuildId, SoundFileId, UserId},
    sound_file::SoundFile,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuildFile {
    pub guild_id: GuildId,
    pub file_id: SoundFileId,
    pub time_added: NaiveDateTime,
    #[serde(skip)]
    pub is_deleted: bool,
    pub sound_file: Option<SoundFile>,
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
            ON CONFLICT (guild_id, file_id)
            DO UPDATE
            SET is_deleted = false;
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
            UPDATE guild_file
            SET is_deleted = true
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
            AND guild_file.is_deleted = false
            ",
            guild_id.0 as i64,
        )
        .fetch_all(&mut *transaction)
        .await?;

        let guild_files = records
            .into_iter()
            .map(|r| {
                let owner = r.owner.map(|o| UserId(o as u64));
                SoundFile {
                    id: SoundFileId(r.id as u64),
                    owner,
                    display_name: r.display_name,
                    time_added: Some(r.time_added),
                    is_public: r.is_public.unwrap_or(false),
                    is_deleted: r.is_deleted.unwrap_or(false),
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
            SELECT
            guild_file.guild_id,
            guild_file.file_id,
            guild_file.time_added,
            guild_file.is_deleted,
            files.display_name,
            files.is_deleted as file_is_deleted
            FROM guild_file
            INNER JOIN files ON files.id = $2
            WHERE guild_file.guild_id = $1 AND guild_file.file_id = $2
            AND guild_file.is_deleted = false
            ",
            guild_id.0 as i64,
            file_id.0 as i64
        )
        .fetch_optional(&mut *transaction)
        .await?
        {
            Some(r) => {
                return Ok(Some(Self {
                    guild_id: GuildId(r.guild_id as u64),
                    file_id: SoundFileId(r.file_id as u64),
                    time_added: r.time_added,
                    is_deleted: r.is_deleted.unwrap_or(false),
                    sound_file: Some(SoundFile {
                        id: SoundFileId(r.file_id as u64),
                        display_name: r.display_name,
                        // we don't care about other fields
                        owner: None,
                        is_public: false,
                        is_deleted: false,
                        time_added: None,
                    }),
                }));
            }
            None => return Ok(None),
        }
    }

    pub async fn get_matching_guilds_for_file(
        guilds: &[Guild],
        file_id: &SoundFileId,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<HashSet<Guild>, KekServerError> {
        let guild_ids = guilds
            .iter()
            .map(|guild| guild.id.0 as i64)
            .collect::<Vec<i64>>();
        let records = sqlx::query!(
            "
            SELECT
                guild_id,
                file_id,
                guild_file.time_added,
                is_deleted,
                guild.id,
                guild.name,
                guild.icon,
                guild.icon_hash,
                guild.time_added as guild_time_added 
            FROM guild_file
            INNER JOIN guild ON id = ANY($1)
            AND guild_id = ANY($1)
            AND file_id = $2
            AND guild_file.is_deleted = false
            ",
            &guild_ids,
            file_id.0 as i64
        )
        .fetch_all(&mut *transaction)
        .await?;

        let guilds = records
            .into_iter()
            .map(|r| Guild {
                id: GuildId(r.guild_id as u64),
                name: r.name,
                icon: r.icon,
                icon_hash: r.icon_hash,
                time_added: Some(r.guild_time_added),
            })
            .collect();
        return Ok(guilds);
    }

    pub async fn get_users_enabled_files_for_guild(
        user: &UserId,
        guild: &GuildId,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<HashSet<SoundFile>, KekServerError> {
        let records = sqlx::query!(
            "
            SELECT 
                id,
                display_name,
                owner,
                files.time_added as file_time_added,
                files.is_public as file_is_public,
                files.is_deleted as file_is_deleted
            FROM guild_file
            INNER JOIN files ON files.id = guild_file.file_id 
            AND owner = $1 
            AND guild_id = $2
            AND guild_file.is_deleted = false
            ",
            user.0 as i64,
            guild.0 as i64
        )
        .fetch_all(&mut *transaction)
        .await?;

        let enabled_sounds = records
            .into_iter()
            .map(|r| {
                let owner = r.owner.map(|o| UserId(o as u64));
                SoundFile {
                    id: SoundFileId(r.id as u64),
                    display_name: r.display_name,
                    owner,
                    time_added: Some(r.file_time_added),
                    is_public: r.file_is_public.unwrap_or(false),
                    is_deleted: r.file_is_deleted.unwrap_or(false),
                }
            })
            .collect();

        return Ok(enabled_sounds);
    }

    pub async fn bulk_insert(
        guild_ids: &[GuildId],
        file_ids: &[SoundFileId],
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<(), KekServerError> {
        let guild_ids = guild_ids.iter().map(|g| g.0 as i64).collect::<Vec<i64>>();
        let file_ids = file_ids.iter().map(|f| f.0 as i64).collect::<Vec<i64>>();
        sqlx::query!(
            "
            INSERT INTO guild_file (guild_id, file_id)
            SELECT guild_id, file_id FROM UNNEST($1::bigint[]) as guild_id, UNNEST($2::bigint[]) as file_id
            ",
            &guild_ids,
            &file_ids
        )
        .execute(&mut *transaction)
        .await?;
        return Ok(());
    }
}
