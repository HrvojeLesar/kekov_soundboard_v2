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
    guild_id: GuildId,
    file_id: SoundFileId,
    time_added: NaiveDateTime,
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
                    time_added: guild_file.time_added,
                }))
            }
            None => return Ok(None),
        }
    }

    pub async fn get_matching_guilds_for_file(
        guilds: &Vec<Guild>,
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
            INNER JOIN guild ON id = ANY($1) AND guild_id = ANY($1) AND file_id = $2
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
                time_added: r.guild_time_added,
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
            SELECT id, display_name, owner FROM guild_file
            INNER JOIN files ON files.id = guild_file.file_id 
            AND owner = $1 
            AND guild_id = $2;
            ",
            user.0 as i64,
            guild.0 as i64
        )
        .fetch_all(&mut *transaction)
        .await?;

        let enabled_sounds = records
            .into_iter()
            .map(|r| {
                let owner = match r.owner {
                    Some(o) => Some(UserId(o as u64)),
                    None => None,
                };

                SoundFile {
                    id: SoundFileId(r.id as u64),
                    display_name: r.display_name,
                    owner,
                }
            })
            .collect();

        return Ok(enabled_sounds);
    }

    pub async fn bulk_insert(
        guild_ids: &Vec<GuildId>,
        file_ids: &Vec<SoundFileId>,
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
