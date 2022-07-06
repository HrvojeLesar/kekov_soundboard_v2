use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};

use crate::error::errors::KekServerError;

use super::{ids::{SoundFileId, UserId}, postgres_like_escape};

pub const MAX_LIMIT: i64 = 200;

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct SoundFile {
    pub id: SoundFileId,
    pub display_name: Option<String>,
    #[serde(skip)]
    pub owner: Option<UserId>,
    pub time_added: NaiveDateTime,
    #[serde(skip)]
    pub is_deleted: bool,
    #[serde(default)]
    pub is_public: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct SoundFilePartial {
    pub id: SoundFileId,
    pub display_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FilesAndCount {
    pub count: i64,
    pub max: i64,
    pub files: Vec<SoundFile>,
}

impl SoundFile {
    // TODO: remove this
    pub fn new(
        id: SoundFileId,
        display_name: String,
        owner: Option<UserId>,
        is_public: bool,
    ) -> Self {
        let now = Utc::now().naive_utc();
        return Self {
            id,
            display_name: Some(display_name),
            owner,
            time_added: now,
            is_deleted: false,
            is_public,
        };
    }

    pub async fn insert(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Self, KekServerError> {
        let r = sqlx::query!(
            "
            INSERT INTO files (id, display_name, owner, is_public)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            ",
            self.id.0 as i64,
            self.display_name,
            self.owner.as_ref().map(|o| o.0 as i64),
            self.is_public
        )
        .fetch_one(&mut *transaction)
        .await?;
        return Ok(Self {
            id: r.id.into(),
            display_name: r.display_name,
            owner: r.owner.map(|o| o.into()),
            time_added: r.time_added,
            is_deleted: r.is_deleted.unwrap_or(false),
            is_public: r.is_public.unwrap_or(false),
        });
    }

    pub async fn toggle_visibility(
        id: &SoundFileId,
        owner: &UserId,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Self, KekServerError> {
        let r = sqlx::query!(
            "
            UPDATE files
            SET is_public = NOT is_public
            WHERE id = $1 AND owner = $2
            RETURNING *
            ",
            id.0 as i64,
            owner.0 as i64
        )
        .fetch_one(&mut *transaction)
        .await?;
        return Ok(Self {
            id: r.id.into(),
            owner: r.owner.map(|o| o.into()),
            display_name: r.display_name,
            time_added: r.time_added,
            is_public: r.is_public.unwrap_or(false),
            is_deleted: r.is_deleted.unwrap_or(false),
        });
    }

    pub async fn delete(
        id: &SoundFileId,
        owner: &UserId,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Self, KekServerError> {
        let r = sqlx::query!(
            "
            UPDATE files
            SET is_deleted = true
            WHERE id = $1 AND owner = $2
            RETURNING *
            ",
            id.0 as i64,
            owner.0 as i64
        )
        .fetch_one(&mut *transaction)
        .await?;
        return Ok(Self {
            id: r.id.into(),
            owner: r.owner.map(|o| o.into()),
            display_name: r.display_name,
            time_added: r.time_added,
            is_public: r.is_public.unwrap_or(false),
            is_deleted: r.is_deleted.unwrap_or(false),
        });
    }

    pub async fn delete_multiple(
        ids: &[SoundFileId],
        owner: &UserId,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Vec<Self>, KekServerError> {
        let ids = ids.iter().map(|id| id.0 as i64).collect::<Vec<i64>>();
        let records = sqlx::query!(
            "
            UPDATE files
            SET is_deleted = true
            WHERE id = ANY($1) AND owner = $2
            RETURNING *
            ",
            &ids,
            owner.0 as i64
        )
        .fetch_all(&mut *transaction)
        .await?;
        let rows_deleted = records
            .into_iter()
            .map(|r| Self {
                id: r.id.into(),
                owner: r.owner.map(|o| o.into()),
                display_name: r.display_name,
                time_added: r.time_added,
                is_public: r.is_public.unwrap_or(false),
                is_deleted: r.is_deleted.unwrap_or(false),
            })
            .collect::<Vec<SoundFile>>();
        return Ok(rows_deleted);
    }

    pub async fn get_file(
        id: &SoundFileId,
        owner_id: &UserId,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<Self>, KekServerError> {
        match sqlx::query!(
            "
            SELECT * FROM files
            WHERE id = $1 AND is_deleted = false
            AND (is_public = true OR owner = $2)
            ",
            id.0 as i64,
            owner_id.0 as i64
        )
        .fetch_optional(&mut *transaction)
        .await?
        {
            Some(r) => {
                return Ok(Some(Self {
                    id: SoundFileId(r.id as u64),
                    owner: r.owner.map(|o| UserId(o as u64)),
                    display_name: r.display_name,
                    time_added: r.time_added,
                    is_public: r.is_public.unwrap_or(false),
                    is_deleted: r.is_deleted.unwrap_or(false),
                }));
            }
            None => return Ok(None),
        }
    }

    pub async fn get_user_files(
        user: &UserId,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Vec<Self>, KekServerError> {
        let records = sqlx::query!(
            "
            SELECT * FROM files
            WHERE owner = $1 AND is_deleted = false
            ",
            user.0 as i64
        )
        .fetch_all(&mut *transaction)
        .await?;
        let files = records
            .into_iter()
            .map(|r| Self {
                id: SoundFileId(r.id as u64),
                owner: r.owner.map(|o| UserId(o as u64)),
                display_name: r.display_name,
                time_added: r.time_added,
                is_public: r.is_public.unwrap_or(false),
                is_deleted: r.is_deleted.unwrap_or(false),
            })
            .collect();
        return Ok(files);
    }

    pub async fn get_public_files(
        limit: i64,
        page: i64,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<FilesAndCount, KekServerError> {
        let offset = if page < 1 { 0 } else { page - 1 };
        let records = sqlx::query!(
            "
            SELECT * FROM files
            WHERE is_public = true AND is_deleted = false
            LIMIT $1 OFFSET $2
            ",
            if limit > MAX_LIMIT { MAX_LIMIT } else { limit },
            limit * offset
        )
        .fetch_all(&mut *transaction)
        .await?;

        let count = sqlx::query!(
            "
            SELECT COUNT(*) as count FROM files
            WHERE is_public = true AND is_deleted = false
            "
        )
        .fetch_one(&mut *transaction)
        .await?
        .count
        .unwrap_or(0);

        let files = records
            .into_iter()
            .map(|r| Self {
                id: SoundFileId(r.id as u64),
                owner: r.owner.map(|o| UserId(o as u64)),
                display_name: r.display_name,
                time_added: r.time_added,
                is_public: r.is_public.unwrap_or(true),
                is_deleted: r.is_deleted.unwrap_or(false),
            })
            .collect();
        return Ok(FilesAndCount { count, files, max: MAX_LIMIT });
    }

    pub async fn get_public_files_search(
        limit: i64,
        page: i64,
        search: String,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<FilesAndCount, KekServerError> {
        let offset = if page < 1 { 0 } else { page - 1 };

        let mut search = postgres_like_escape(search);
        search.push('%');
        search.insert(0, '%');

        let records = sqlx::query!(
            "
            SELECT * FROM files
            WHERE is_public = true AND is_deleted = false
            AND display_name ILIKE $3
            LIMIT $1 OFFSET $2
            ",
            if limit > MAX_LIMIT { MAX_LIMIT } else { limit },
            limit * offset,
            &search
        )
        .fetch_all(&mut *transaction)
        .await?;
        let count = sqlx::query!(
            "
            SELECT COUNT(*) as count FROM files
            WHERE is_public = true AND is_deleted = false
            AND display_name ILIKE $1
            ",
            &search
        )
        .fetch_one(&mut *transaction)
        .await?
        .count
        .unwrap_or(0);

        let files = records
            .into_iter()
            .map(|r| Self {
                id: SoundFileId(r.id as u64),
                owner: r.owner.map(|o| UserId(o as u64)),
                display_name: r.display_name,
                time_added: r.time_added,
                is_public: r.is_public.unwrap_or(true),
                is_deleted: r.is_deleted.unwrap_or(false),
            })
            .collect();
        return Ok(FilesAndCount { count, files, max: MAX_LIMIT });
    }
}

#[cfg(test)]
mod tests {
    use sqlx::Connection;
    use uuid::Uuid;

    use crate::{
        database::tests_db_helper::db_connection,
        models::ids::SoundFileId,
        utils::test_utils::{
            insert_file_test_util, insert_random_file_test_util, insert_user_test_util,
            PublicDeleted,
        },
    };

    use super::{SoundFile, MAX_LIMIT};

    #[actix_web::test]
    async fn test_insert_sound_file() {
        let mut connection = db_connection().await;
        let mut transaction = connection.begin().await.unwrap();
        let is_public = false;

        let sound_file_id = SoundFileId(Uuid::new_v4().as_u128() as u64);
        let owner = insert_user_test_util(&mut transaction).await;
        let generated_sound_file = SoundFile::new(
            sound_file_id,
            "Test file".to_owned(),
            Some(owner.id),
            is_public,
        );
        let sound_file = generated_sound_file.insert(&mut transaction).await.unwrap();
        transaction.commit().await.unwrap();

        assert_eq!(sound_file.id, generated_sound_file.id);
        assert_eq!(sound_file.display_name, generated_sound_file.display_name);
        assert_eq!(sound_file.owner, generated_sound_file.owner);
        assert_eq!(sound_file.is_deleted, false);
        assert_eq!(sound_file.is_public, is_public);
    }

    #[actix_web::test]
    async fn test_toggle_visibility() {
        let mut connection = db_connection().await;
        let mut transaction = connection.begin().await.unwrap();

        let sound_file = insert_random_file_test_util(&mut transaction).await;
        let is_public_before_toggle = sound_file.is_public;
        let sound_file = SoundFile::toggle_visibility(
            &sound_file.id,
            &sound_file.owner.unwrap(),
            &mut transaction,
        )
        .await
        .unwrap();
        transaction.commit().await.unwrap();

        assert_ne!(sound_file.is_public, is_public_before_toggle);
    }

    #[actix_web::test]
    async fn test_delete_sound_file() {
        let mut connection = db_connection().await;
        let mut transaction = connection.begin().await.unwrap();

        let sound_file = insert_random_file_test_util(&mut transaction).await;
        let deleted_sound_file =
            SoundFile::delete(&sound_file.id, &sound_file.owner.unwrap(), &mut transaction)
                .await
                .unwrap();
        transaction.commit().await.unwrap();

        assert_eq!(deleted_sound_file.is_deleted, true);
        assert_ne!(deleted_sound_file.is_deleted, sound_file.is_deleted);
    }

    #[actix_web::test]
    async fn test_delete_multiple_sound_files() {
        let mut connection = db_connection().await;
        let mut transaction = connection.begin().await.unwrap();

        let owner = insert_user_test_util(&mut transaction).await;

        let mut files = vec![];
        let mut file_ids = vec![];
        for _ in 0..10 {
            let sound_file = insert_file_test_util(&owner.id, None, &mut transaction).await;
            file_ids.push(sound_file.id.clone());
            files.push(sound_file);
        }

        let deleted_files = SoundFile::delete_multiple(&file_ids, &owner.id, &mut transaction)
            .await
            .unwrap();

        transaction.commit().await.unwrap();

        assert_eq!(deleted_files.len(), files.len());
        for file in &deleted_files {
            assert!(files.iter().find(|f| f.id == file.id).is_some());
        }
    }

    #[actix_web::test]
    async fn test_get_file() {
        let mut connection = db_connection().await;
        let mut transaction = connection.begin().await.unwrap();

        let sound_file = insert_random_file_test_util(&mut transaction).await;
        let gotten_file = SoundFile::get_file(
            &sound_file.id,
            &sound_file.owner.clone().unwrap(),
            &mut transaction,
        )
        .await
        .unwrap()
        .unwrap();

        transaction.commit().await.unwrap();

        assert_eq!(gotten_file.id, sound_file.id);
        assert_eq!(gotten_file.display_name, sound_file.display_name);
        assert_eq!(gotten_file.owner, sound_file.owner);
        assert_eq!(
            gotten_file.time_added.timestamp(),
            sound_file.time_added.timestamp()
        );
        assert_eq!(gotten_file.is_deleted, sound_file.is_deleted);
        assert_eq!(gotten_file.is_public, sound_file.is_public);
    }

    #[actix_web::test]
    async fn test_get_user_files() {
        let mut connection = db_connection().await;
        let mut transaction = connection.begin().await.unwrap();

        let owner = insert_user_test_util(&mut transaction).await;
        let other_owner = insert_user_test_util(&mut transaction).await;

        let mut files = vec![];
        for i in 0..10 {
            if i != 2 {
                if i % 2 == 0 {
                    files.push(insert_file_test_util(&owner.id, None, &mut transaction).await);
                } else {
                    insert_file_test_util(
                        &owner.id,
                        Some(PublicDeleted {
                            is_deleted: true,
                            is_public: false,
                        }),
                        &mut transaction,
                    )
                    .await;
                }
            } else {
                insert_file_test_util(&other_owner.id, None, &mut transaction).await;
            }
        }
        let gotten_files = SoundFile::get_user_files(&owner.id, &mut transaction)
            .await
            .unwrap();

        transaction.commit().await.unwrap();

        assert_eq!(gotten_files.len(), files.len());
        for gotten_file in &gotten_files {
            assert_eq!(gotten_file.is_deleted, false);
        }
    }

    #[actix_web::test]
    async fn test_get_public_files() {
        let mut connection = db_connection().await;
        let mut transaction = connection.begin().await.unwrap();

        let owner = insert_user_test_util(&mut transaction).await;
        let other_owner = insert_user_test_util(&mut transaction).await;
        let mut random_files = vec![];

        for i in 0..MAX_LIMIT * 3 {
            if i % 2 == 0 {
                random_files.push(
                    insert_file_test_util(
                        &owner.id,
                        Some(PublicDeleted {
                            is_deleted: false,
                            is_public: true,
                        }),
                        &mut transaction,
                    )
                    .await,
                );
            } else {
                random_files.push(
                    insert_file_test_util(
                        &other_owner.id,
                        Some(PublicDeleted {
                            is_deleted: true,
                            is_public: true,
                        }),
                        &mut transaction,
                    )
                    .await,
                );
            }
        }

        let public_files_page_0 = SoundFile::get_public_files(MAX_LIMIT, 0, &mut transaction)
            .await
            .unwrap();

        let public_files_page_1 = SoundFile::get_public_files(MAX_LIMIT, 1, &mut transaction)
            .await
            .unwrap();

        let public_files_page_2 = SoundFile::get_public_files(MAX_LIMIT, 2, &mut transaction)
            .await
            .unwrap();

        let public_files_limit_100 = SoundFile::get_public_files(100, 1, &mut transaction)
            .await
            .unwrap();

        let public_files_over_max =
            SoundFile::get_public_files(MAX_LIMIT + MAX_LIMIT, 1, &mut transaction)
                .await
                .unwrap();

        let public_files_limit_100_page_2 = SoundFile::get_public_files(100, 2, &mut transaction)
            .await
            .unwrap();

        assert_eq!(public_files_page_0.files, public_files_page_1.files);
        assert_ne!(public_files_page_0.files, public_files_page_2.files);
        assert_eq!(public_files_limit_100.files.len(), 100usize);
        assert_eq!(public_files_over_max.files.len(), MAX_LIMIT as usize);

        assert_eq!(public_files_limit_100_page_2.files.len(), 100usize);
        assert_eq!(
            public_files_limit_100_page_2.files,
            public_files_page_1
                .clone()
                .files
                .into_iter()
                .skip(100)
                .collect::<Vec<SoundFile>>()
        );

        for pf in &public_files_page_0.files {
            assert_eq!(pf.is_public, true);
            assert_eq!(pf.is_deleted, false);
        }

        transaction.commit().await.unwrap();
    }
}
