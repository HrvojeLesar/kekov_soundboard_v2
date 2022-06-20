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
            SELECT files.*, guild_file.time_added as gf_time_added FROM files
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
                    time_added: Some(r.gf_time_added),
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
                        // WARN: we don't care about other fields
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

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;

    use crate::{
        database::tests_db_helper::DB_POOL,
        models::{
            guild::Guild,
            ids::{GuildId, SoundFileId, UserId},
            sound_file::{self, SoundFile},
            user::User,
        },
    };

    use super::GuildFile;

    #[actix_web::test]
    async fn test_insert_guild_file() {
        let guild_id = GuildId(Uuid::new_v4().as_u128() as u64);
        let sound_file_id = SoundFileId(Uuid::new_v4().as_u128() as u64);
        let is_public = true;
        let is_deleted = false;
        let now = Utc::now().naive_utc();

        let mut transaction = DB_POOL.begin().await.unwrap();
        let sound_file = SoundFile {
            id: sound_file_id.clone(),
            is_public: is_public.clone(),
            is_deleted: is_deleted.clone(),
            display_name: Some("Test".to_string()),
            time_added: Some(now),
            owner: None,
        };

        Guild::insert_guild(&guild_id, "Test", None, None, &mut transaction)
            .await
            .unwrap();
        sound_file.insert(&mut transaction).await.unwrap();

        GuildFile::insert_guild_file(&guild_id, &sound_file_id, &mut transaction)
            .await
            .unwrap();

        let guild_file = GuildFile::get_guild_file(&guild_id, &sound_file_id, &mut transaction)
            .await
            .unwrap();
        transaction.commit().await.unwrap();

        assert!(guild_file.is_some());
        let guild_file = guild_file.unwrap();
        let guild_sound_file = guild_file.sound_file.unwrap();
        assert_eq!(guild_file.guild_id, guild_id);
        assert_eq!(guild_file.file_id, sound_file_id);
        assert_eq!(guild_file.is_deleted, false);
        assert_ne!(guild_file.time_added.timestamp(), now.timestamp());
        assert_eq!(guild_sound_file.id, sound_file.id);
        assert_eq!(guild_sound_file.display_name, sound_file.display_name);
    }

    #[actix_web::test]
    async fn test_insert_guild_file_do_update() {
        let guild_id = GuildId(Uuid::new_v4().as_u128() as u64);
        let sound_file_id = SoundFileId(Uuid::new_v4().as_u128() as u64);
        let is_public = true;
        let is_deleted = false;
        let now = Utc::now().naive_utc();

        let mut transaction = DB_POOL.begin().await.unwrap();

        let sound_file = SoundFile {
            id: sound_file_id.clone(),
            is_public: is_public.clone(),
            is_deleted: is_deleted.clone(),
            display_name: Some("Test".to_string()),
            time_added: Some(now),
            owner: None,
        };
        Guild::insert_guild(&guild_id, "Test", None, None, &mut transaction)
            .await
            .unwrap();
        sound_file.insert(&mut transaction).await.unwrap();

        GuildFile::insert_guild_file(&guild_id, &sound_file_id, &mut transaction)
            .await
            .unwrap();

        GuildFile::delete_guild_file(&guild_id, &sound_file_id, &mut transaction)
            .await
            .unwrap();

        GuildFile::insert_guild_file(&guild_id, &sound_file_id, &mut transaction)
            .await
            .unwrap();

        let guild_file = GuildFile::get_guild_file(&guild_id, &sound_file_id, &mut transaction)
            .await
            .unwrap();
        transaction.commit().await.unwrap();

        assert!(guild_file.is_some());
        let guild_file = guild_file.unwrap();
        let guild_sound_file = guild_file.sound_file.unwrap();
        assert_eq!(guild_file.guild_id, guild_id);
        assert_eq!(guild_file.file_id, sound_file_id);
        assert_eq!(guild_file.is_deleted, false);
        assert_ne!(guild_file.time_added.timestamp(), now.timestamp());
        assert_eq!(guild_sound_file.id, sound_file.id);
        assert_eq!(guild_sound_file.display_name, sound_file.display_name);
    }

    #[actix_web::test]
    async fn test_delete_guild_file() {
        let guild_id = GuildId(Uuid::new_v4().as_u128() as u64);
        let sound_file_id = SoundFileId(Uuid::new_v4().as_u128() as u64);
        let is_public = true;
        let is_deleted = false;
        let now = Utc::now().naive_utc();

        let mut transaction = DB_POOL.begin().await.unwrap();

        let sound_file = SoundFile {
            id: sound_file_id.clone(),
            is_public: is_public.clone(),
            is_deleted: is_deleted.clone(),
            display_name: Some("Test".to_string()),
            time_added: Some(now),
            owner: None,
        };

        Guild::insert_guild(&guild_id, "Test", None, None, &mut transaction)
            .await
            .unwrap();
        sound_file.insert(&mut transaction).await.unwrap();

        GuildFile::insert_guild_file(&guild_id, &sound_file_id, &mut transaction)
            .await
            .unwrap();

        GuildFile::delete_guild_file(&guild_id, &sound_file_id, &mut transaction)
            .await
            .unwrap();

        let guild_file_none =
            GuildFile::get_guild_file(&guild_id, &sound_file_id, &mut transaction)
                .await
                .unwrap();

        let guild_file = match sqlx::query!(
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
            ",
            guild_id.0 as i64,
            sound_file.id.0 as i64
        )
        .fetch_one(&mut transaction)
        .await
        {
            Ok(r) => {
                GuildFile {
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
                }
            }
            Err(e) => panic!("{}", e.to_string()),
        };

        transaction.commit().await.unwrap();

        assert!(guild_file_none.is_none());
        assert_eq!(guild_file.is_deleted, true);
    }

    #[actix_web::test]
    async fn test_get_guild_files() {
        let mut sound_files = vec![];
        let now = Utc::now().naive_utc();
        let is_public = false;
        let is_deleted = false;

        let mut transaction = DB_POOL.begin().await.unwrap();
        let guild_id = GuildId(Uuid::new_v4().as_u128() as u64);
        Guild::insert_guild(&guild_id, "Test", None, None, &mut transaction)
            .await
            .unwrap();

        for i in 0..5 {
            let sound_file_id = SoundFileId(Uuid::new_v4().as_u128() as u64);

            let sound_file = SoundFile {
                id: sound_file_id.clone(),
                is_public: is_public.clone(),
                is_deleted: is_deleted.clone(),
                display_name: Some("Test".to_string()),
                time_added: Some(now),
                owner: None,
            };

            sound_file.insert(&mut transaction).await.unwrap();

            GuildFile::insert_guild_file(&guild_id, &sound_file_id, &mut transaction)
                .await
                .unwrap();

            if i == 1 || i == 3 {
                GuildFile::delete_guild_file(&guild_id, &sound_file_id, &mut transaction)
                    .await
                    .unwrap();
            } else {
                sound_files.push(sound_file);
            }
        }

        let guild_files = GuildFile::get_guild_files(&guild_id, &mut transaction)
            .await
            .unwrap();

        transaction.commit().await.unwrap();

        assert_eq!(guild_files.len(), sound_files.len());
        for file in &guild_files {
            let sf = sound_files.iter().find(|sf| sf.id == file.id).unwrap();
            assert_eq!(sf.display_name, file.display_name);
            assert_eq!(sf.is_deleted, file.is_deleted);
            assert_eq!(sf.is_public, file.is_public);
        }
    }

    #[actix_web::test]
    async fn test_get_guild_file() {
        let mut transaction = DB_POOL.begin().await.unwrap();
        let now = Utc::now().naive_utc();

        let guild_id = GuildId(Uuid::new_v4().as_u128() as u64);
        Guild::insert_guild(&guild_id, "Test", None, None, &mut transaction)
            .await
            .unwrap();

        let sound_file_id = SoundFileId(Uuid::new_v4().as_u128() as u64);

        let sound_file = SoundFile {
            id: sound_file_id.clone(),
            is_public: false,
            is_deleted: false,
            display_name: Some("Test".to_string()),
            time_added: None,
            owner: None,
        };

        sound_file.insert(&mut transaction).await.unwrap();

        sqlx::query!(
            "
            INSERT INTO guild_file (guild_id, file_id, time_added)
            VALUES ($1, $2, $3)
            ",
            guild_id.0 as i64,
            sound_file_id.0 as i64,
            now,
        )
        .execute(&mut transaction)
        .await
        .unwrap();

        let guild_file = GuildFile::get_guild_file(&guild_id, &sound_file_id, &mut transaction)
            .await
            .unwrap();

        transaction.commit().await.unwrap();

        assert!(guild_file.is_some());
        let guild_file = guild_file.unwrap();
        assert_eq!(guild_file.guild_id, guild_id);
        assert_eq!(guild_file.file_id, sound_file_id);
        assert_eq!(guild_file.time_added.timestamp(), now.timestamp());
        assert_eq!(guild_file.is_deleted, false);
        assert_eq!(guild_file.sound_file.clone().unwrap().id, sound_file_id);
    }

    #[actix_web::test]
    async fn test_get_matching_guilfs_for_file() {
        let mut transaction = DB_POOL.begin().await.unwrap();
        let now = Utc::now().naive_utc();

        let sound_file_id = SoundFileId(Uuid::new_v4().as_u128() as u64);

        let sound_file = SoundFile {
            id: sound_file_id.clone(),
            is_public: false,
            is_deleted: false,
            display_name: Some("Test".to_string()),
            time_added: None,
            owner: None,
        };
        sound_file.insert(&mut transaction).await.unwrap();

        let mut guilds = vec![];
        for i in 0..5 {
            let guild_id = GuildId(Uuid::new_v4().as_u128() as u64);
            Guild::insert_guild(&guild_id, "Test", None, None, &mut transaction)
                .await
                .unwrap();
            if i != 2 {
                GuildFile::insert_guild_file(&guild_id, &sound_file_id, &mut transaction)
                    .await
                    .unwrap();
            }
            guilds.push(Guild {
                id: guild_id,
                name: "Test".to_string(),
                time_added: Some(now),
                icon_hash: None,
                icon: None,
            });
        }

        let matching_guilds =
            GuildFile::get_matching_guilds_for_file(&guilds, &sound_file_id, &mut transaction)
                .await
                .unwrap();

        transaction.commit().await.unwrap();

        assert_eq!(matching_guilds.len(), guilds.len() - 1);
        for m_guild in matching_guilds.iter() {
            let guild = guilds.iter().find(|g| g.id == m_guild.id);
            assert!(guild.is_some());
            let guild = guild.unwrap();
            assert_eq!(m_guild.id, guild.id);
            assert_eq!(
                m_guild.time_added.unwrap().timestamp(),
                guild.time_added.unwrap().timestamp()
            );
        }
    }

    #[actix_web::test]
    async fn test_get_users_enabled_files_for_guild() {
        let user_id = UserId(Uuid::new_v4().as_u128() as u64);
        let guild_id = GuildId(Uuid::new_v4().as_u128() as u64);
        let other_guild_id = GuildId(Uuid::new_v4().as_u128() as u64);
        let mut inserted_files = vec![];

        let mut transaction = DB_POOL.begin().await.unwrap();

        User::insert_user(&user_id, "Test username", None, &mut transaction)
            .await
            .unwrap();

        Guild::insert_guild(&guild_id, "Test", None, None, &mut transaction)
            .await
            .unwrap();

        Guild::insert_guild(&other_guild_id, "Test", None, None, &mut transaction)
            .await
            .unwrap();

        for i in 0..5 {
            let sound_file_id = SoundFileId(Uuid::new_v4().as_u128() as u64);
            let sound_file = SoundFile {
                id: sound_file_id.clone(),
                is_public: false,
                is_deleted: false,
                display_name: Some("Test".to_string()),
                time_added: None,
                owner: Some(user_id.clone()),
            };

            sound_file.insert(&mut transaction).await.unwrap();

            if i != 2 {
                GuildFile::insert_guild_file(&guild_id, &sound_file_id, &mut transaction)
                    .await
                    .unwrap();
            } else {
                GuildFile::insert_guild_file(&other_guild_id, &sound_file_id, &mut transaction)
                    .await
                    .unwrap();
            }
            inserted_files.push(sound_file);
        }

        let files =
            GuildFile::get_users_enabled_files_for_guild(&user_id, &guild_id, &mut transaction)
                .await
                .unwrap();

        transaction.commit().await.unwrap();

        assert_eq!(files.len(), inserted_files.len() - 1);
        for file in files.iter() {
            let in_f = inserted_files.iter().find(|f| f.id == file.id);
            assert!(in_f.is_some());
            let in_f = in_f.unwrap();
            assert_eq!(file.id, in_f.id);
            assert_eq!(file.display_name, in_f.display_name);
            assert_eq!(file.owner, in_f.owner);
        }
    }

    #[actix_web::test]
    async fn test_bulk_insert() {
        let mut transaction = DB_POOL.begin().await.unwrap();
        let mut guild_ids = vec![];
        let mut file_ids = vec![];
        for _ in 0..5 {
            let guild_id = GuildId(Uuid::new_v4().as_u128() as u64);
            Guild::insert_guild(&guild_id, "Test guild", None, None, &mut transaction)
                .await
                .unwrap();

            let sound_file_id = SoundFileId(Uuid::new_v4().as_u128() as u64);
            let sound_file = SoundFile {
                id: sound_file_id.clone(),
                is_public: false,
                is_deleted: false,
                display_name: Some("Test".to_string()),
                time_added: None,
                owner: None,
            };
            sound_file.insert(&mut transaction).await.unwrap();

            guild_ids.push(guild_id);
            file_ids.push(sound_file_id);
        }

        let res = GuildFile::bulk_insert(&guild_ids, &file_ids, &mut transaction).await.unwrap();
        transaction.commit().await.unwrap();
        
        assert_eq!(res, ());
    }
}
