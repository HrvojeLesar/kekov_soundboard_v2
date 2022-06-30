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
    ) -> Result<Option<Self>, KekServerError> {
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
        return Self::get_guild_file(guild_id, file_id, transaction).await;
    }

    pub async fn delete_guild_file(
        guild_id: &GuildId,
        file_id: &SoundFileId,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<Self>, KekServerError> {
        let guild_file = Self::get_guild_file(guild_id, file_id, transaction).await?;
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
        return Ok(guild_file);
    }

    pub async fn get_guild_files(
        guild_id: &GuildId,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Vec<Self>, KekServerError> {
        let records = sqlx::query!(
            "
            SELECT 
                files.*,
                guild_file.guild_id,
                guild_file.file_id,
                guild_file.time_added as gf_time_added,
                guild_file.is_deleted as gf_is_deleted
            FROM files
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
            .map(|r| GuildFile {
                guild_id: r.guild_id.into(),
                file_id: r.file_id.into(),
                time_added: r.gf_time_added,
                is_deleted: r.gf_is_deleted.unwrap_or(false),
                sound_file: Some(SoundFile {
                    id: r.id.into(),
                    owner: r.owner.map(|o| o.into()),
                    display_name: r.display_name,
                    time_added: r.time_added,
                    is_public: r.is_public.unwrap_or(false),
                    is_deleted: r.is_deleted.unwrap_or(false),
                }),
            })
            .collect::<Vec<Self>>();

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
                files.*,
                guild_file.guild_id,
                guild_file.file_id,
                guild_file.time_added as gf_time_added,
                guild_file.is_deleted as gf_is_deleted
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
                    guild_id: r.guild_id.into(),
                    file_id: r.file_id.into(),
                    time_added: r.gf_time_added,
                    is_deleted: r.gf_is_deleted.unwrap_or(false),
                    sound_file: Some(SoundFile {
                        id: r.file_id.into(),
                        display_name: r.display_name,
                        owner: r.owner.map(|o| o.into()),
                        time_added: r.time_added,
                        is_public: r.is_public.unwrap_or(false),
                        is_deleted: r.is_deleted.unwrap_or(false),
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
                guild.active,
                guild.time_added as guild_time_added 
            FROM guild_file
            INNER JOIN guild ON id = guild_file.guild_id
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
                id: r.guild_id.into(),
                name: r.name,
                time_added: r.guild_time_added,
                active: r.active,
            })
            .collect();
        return Ok(guilds);
    }

    pub async fn get_users_enabled_files_for_guild(
        user_id: &UserId,
        guild_id: &GuildId,
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
            user_id.0 as i64,
            guild_id.0 as i64
        )
        .fetch_all(&mut *transaction)
        .await?;

        let enabled_sounds = records
            .into_iter()
            .map(|r| SoundFile {
                id: r.id.into(),
                display_name: r.display_name,
                owner: r.owner.map(|o| o.into()),
                time_added: r.file_time_added,
                is_public: r.file_is_public.unwrap_or(false),
                is_deleted: r.file_is_deleted.unwrap_or(false),
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

#[cfg(test)]
mod tests {
    use sqlx::{Connection, Postgres, Transaction};

    use crate::{
        database::tests_db_helper::db_connection,
        models::{
            ids::{GuildId, SoundFileId, UserId},
            sound_file::SoundFile,
        },
        utils::test_utils::{
            insert_file_test_util, insert_guild_file_test_util, insert_guild_test_util,
            insert_random_file_test_util, insert_random_guild_file_test_util,
            insert_user_test_util,
        },
    };

    use super::GuildFile;

    async fn get_guild_file(
        guild_id: &GuildId,
        file_id: &SoundFileId,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> GuildFile {
        let guild_file = sqlx::query!(
            "
            SELECT
                files.*,
                guild_file.guild_id,
                guild_file.file_id,
                guild_file.time_added as gf_time_added,
                guild_file.is_deleted as gf_is_deleted
            FROM guild_file
            INNER JOIN files ON files.id = $2
            WHERE guild_file.guild_id = $1 AND guild_file.file_id = $2
            ",
            guild_id.0 as i64,
            file_id.0 as i64
        )
        .fetch_optional(&mut *transaction)
        .await
        .unwrap()
        .map(|r| GuildFile {
            guild_id: GuildId(r.guild_id as u64),
            file_id: SoundFileId(r.file_id as u64),
            time_added: r.gf_time_added,
            is_deleted: r.gf_is_deleted.unwrap_or(false),
            sound_file: Some(SoundFile {
                id: SoundFileId(r.id as u64),
                owner: r.owner.map(|o| UserId(o as u64)),
                display_name: r.display_name,
                time_added: r.time_added,
                is_public: r.is_public.unwrap_or(false),
                is_deleted: r.is_deleted.unwrap_or(false),
            }),
        })
        .unwrap();

        return guild_file;
    }

    async fn delete_guild_file(
        guild_file: &GuildFile,
        transaction: &mut Transaction<'_, Postgres>,
    ) {
        sqlx::query!(
            "
            UPDATE guild_file
            SET is_deleted = true
            WHERE guild_id = $1 AND file_id = $2
            ",
            guild_file.guild_id.0 as i64,
            guild_file.file_id.0 as i64,
        )
        .execute(&mut *transaction)
        .await
        .unwrap();
    }

    #[actix_web::test]
    async fn test_insert_guild_file() {
        let mut connection = db_connection().await;
        let mut transaction = connection.begin().await.unwrap();
        let guild = insert_guild_test_util(&mut transaction).await;
        let sound_file = insert_random_file_test_util(&mut transaction).await;

        GuildFile::insert_guild_file(&guild.id, &sound_file.id, &mut transaction)
            .await
            .unwrap();

        let guild_file = get_guild_file(&guild.id, &sound_file.id, &mut transaction).await;
        transaction.commit().await.unwrap();

        assert_eq!(guild_file.guild_id, guild.id);
        assert_eq!(guild_file.file_id, sound_file.id);
        assert_eq!(guild_file.is_deleted, false);
        assert!(guild_file.sound_file.is_some());

        let gf_sound_file = guild_file.sound_file.unwrap();
        assert_eq!(gf_sound_file.id, sound_file.id);
        assert_eq!(gf_sound_file.owner, sound_file.owner);
        assert_eq!(gf_sound_file.display_name, sound_file.display_name);
        assert_eq!(
            gf_sound_file.time_added.timestamp(),
            sound_file.time_added.timestamp()
        );
        assert_eq!(gf_sound_file.is_public, sound_file.is_public);
        assert_eq!(gf_sound_file.is_deleted, sound_file.is_deleted);
    }

    #[actix_web::test]
    async fn test_insert_guild_file_do_update() {
        let mut connection = db_connection().await;
        let mut transaction = connection.begin().await.unwrap();
        let guild_file = insert_random_guild_file_test_util(&mut transaction).await;

        delete_guild_file(&guild_file, &mut transaction).await;

        GuildFile::insert_guild_file(&guild_file.guild_id, &guild_file.file_id, &mut transaction)
            .await
            .unwrap();

        let fetched_guild_file =
            get_guild_file(&guild_file.guild_id, &guild_file.file_id, &mut transaction).await;

        transaction.commit().await.unwrap();

        assert_eq!(fetched_guild_file.guild_id, guild_file.guild_id);
        assert_eq!(fetched_guild_file.file_id, guild_file.file_id);
        assert_eq!(fetched_guild_file.is_deleted, false);
        assert!(fetched_guild_file.sound_file.is_some());

        let gf_sound_file = fetched_guild_file.sound_file.unwrap();
        let test_gf_sound_file = guild_file.sound_file.unwrap();
        assert_eq!(gf_sound_file.id, test_gf_sound_file.id);
        assert_eq!(gf_sound_file.owner, test_gf_sound_file.owner);
        assert_eq!(gf_sound_file.display_name, test_gf_sound_file.display_name);
        assert_eq!(
            gf_sound_file.time_added.timestamp(),
            test_gf_sound_file.time_added.timestamp()
        );
        assert_eq!(gf_sound_file.is_public, test_gf_sound_file.is_public);
        assert_eq!(gf_sound_file.is_deleted, test_gf_sound_file.is_deleted);
    }

    #[actix_web::test]
    async fn test_delete_guild_file() {
        let mut connection = db_connection().await;
        let mut transaction = connection.begin().await.unwrap();
        let guild_file = insert_random_guild_file_test_util(&mut transaction).await;

        GuildFile::delete_guild_file(&guild_file.guild_id, &guild_file.file_id, &mut transaction)
            .await
            .unwrap();

        let deleted_guild_file =
            get_guild_file(&guild_file.guild_id, &guild_file.file_id, &mut transaction).await;

        transaction.commit().await.unwrap();

        assert_eq!(deleted_guild_file.guild_id, guild_file.guild_id);
        assert_eq!(deleted_guild_file.file_id, guild_file.file_id);
        assert_eq!(deleted_guild_file.is_deleted, true);
        assert!(deleted_guild_file.sound_file.is_some());
    }

    #[actix_web::test]
    async fn test_get_guild_files() {
        let mut connection = db_connection().await;
        let mut transaction = connection.begin().await.unwrap();
        let mut sound_files = vec![];

        let guild = insert_guild_test_util(&mut transaction).await;

        for i in 0..5 {
            let sound_file = insert_random_file_test_util(&mut transaction).await;

            let guild_file =
                insert_guild_file_test_util(&guild.id, sound_file.clone(), &mut transaction).await;

            if i == 1 || i == 3 {
                delete_guild_file(&guild_file, &mut transaction).await;
            } else {
                sound_files.push(sound_file);
            }
        }

        let guild_files = GuildFile::get_guild_files(&guild.id, &mut transaction)
            .await
            .unwrap();

        transaction.commit().await.unwrap();

        assert_eq!(guild_files.len(), sound_files.len());
        for file in &guild_files {
            let sound_file = file.sound_file.as_ref().unwrap();
            let sf = sound_files
                .iter()
                .find(|sf| sf.id == sound_file.id)
                .unwrap();
            assert_eq!(sf.id, sound_file.id);
            assert_eq!(sf.display_name, sound_file.display_name);
            assert_eq!(sf.owner, sound_file.owner);
            assert_eq!(sf.time_added.timestamp(), sound_file.time_added.timestamp());
            assert_eq!(sf.is_deleted, sound_file.is_deleted);
            assert_eq!(sf.is_public, sound_file.is_public);
        }
    }

    #[actix_web::test]
    async fn test_get_guild_file() {
        let mut connection = db_connection().await;
        let mut transaction = connection.begin().await.unwrap();

        let test_guild_file = insert_random_guild_file_test_util(&mut transaction).await;

        let guild_file = GuildFile::get_guild_file(
            &test_guild_file.guild_id,
            &test_guild_file.file_id,
            &mut transaction,
        )
        .await
        .unwrap()
        .unwrap();

        transaction.commit().await.unwrap();

        assert_eq!(guild_file.guild_id, test_guild_file.guild_id);
        assert_eq!(guild_file.file_id, test_guild_file.file_id);
        assert_eq!(
            guild_file.time_added.timestamp(),
            test_guild_file.time_added.timestamp()
        );
        assert_eq!(guild_file.is_deleted, false);
        let test_sound_file = test_guild_file.sound_file.unwrap();
        let sound_file = guild_file.sound_file.unwrap();
        assert_eq!(sound_file.id, test_sound_file.id);
        assert_eq!(sound_file.display_name, test_sound_file.display_name);
        assert_eq!(sound_file.owner, test_sound_file.owner);
        assert_eq!(
            sound_file.time_added.timestamp(),
            test_sound_file.time_added.timestamp()
        );
        assert_eq!(sound_file.is_deleted, test_sound_file.is_deleted);
        assert_eq!(sound_file.is_public, test_sound_file.is_public);
    }

    #[actix_web::test]
    async fn test_get_matching_guilds_for_file() {
        let mut connection = db_connection().await;
        let mut transaction = connection.begin().await.unwrap();

        let sound_file = insert_random_file_test_util(&mut transaction).await;

        let mut guilds = vec![];
        for i in 0..5 {
            let guild = insert_guild_test_util(&mut transaction).await;
            if i != 2 {
                insert_guild_file_test_util(&guild.id, sound_file.clone(), &mut transaction).await;
            }
            guilds.push(guild);
        }

        let matching_guilds =
            GuildFile::get_matching_guilds_for_file(&guilds, &sound_file.id, &mut transaction)
                .await
                .unwrap();

        transaction.commit().await.unwrap();

        assert_eq!(matching_guilds.len(), guilds.len() - 1);
        for m_guild in matching_guilds.iter() {
            let guild = guilds.iter().find(|g| g.id == m_guild.id).unwrap();
            assert_eq!(m_guild.id, guild.id);
            assert_eq!(m_guild.name, guild.name);
            assert_eq!(m_guild.time_added.timestamp(), guild.time_added.timestamp());
        }
    }

    #[actix_web::test]
    async fn test_get_users_enabled_files_for_guild() {
        let mut connection = db_connection().await;
        let mut transaction = connection.begin().await.unwrap();
        let mut inserted_files = vec![];

        let user = insert_user_test_util(&mut transaction).await;
        let guild = insert_guild_test_util(&mut transaction).await;
        let other_guild = insert_guild_test_util(&mut transaction).await;

        for i in 0..5 {
            let sound_file = insert_file_test_util(&user.id, None, &mut transaction).await;

            if i != 2 {
                insert_guild_file_test_util(&guild.id, sound_file.clone(), &mut transaction).await;
            } else {
                insert_guild_file_test_util(&other_guild.id, sound_file.clone(), &mut transaction)
                    .await;
            }
            inserted_files.push(sound_file);
        }

        let files =
            GuildFile::get_users_enabled_files_for_guild(&user.id, &guild.id, &mut transaction)
                .await
                .unwrap();

        transaction.commit().await.unwrap();

        assert_eq!(files.len(), inserted_files.len() - 1);
        for file in files.iter() {
            let in_f = inserted_files.iter().find(|f| f.id == file.id).unwrap();
            assert_eq!(file.id, in_f.id);
            assert_eq!(file.display_name, in_f.display_name);
            assert_eq!(file.owner, in_f.owner);
            assert_eq!(file.time_added.timestamp(), in_f.time_added.timestamp());
            assert_eq!(file.is_deleted, in_f.is_deleted);
            assert_eq!(file.is_public, in_f.is_public);
        }
    }

    #[actix_web::test]
    async fn test_bulk_insert() {
        let mut connection = db_connection().await;
        let mut transaction = connection.begin().await.unwrap();
        let mut guild_ids = vec![];
        let mut file_ids = vec![];
        for _ in 0..5 {
            let guild = insert_guild_test_util(&mut transaction).await;
            let sound_file = insert_random_file_test_util(&mut transaction).await;

            guild_ids.push(guild.id);
            file_ids.push(sound_file.id);
        }

        let res = GuildFile::bulk_insert(&guild_ids, &file_ids, &mut transaction)
            .await
            .unwrap();
        transaction.commit().await.unwrap();

        assert_eq!(res, ());
    }
}
