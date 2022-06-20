use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};

use crate::error::errors::KekServerError;

use super::ids::{SoundFileId, UserId};

pub const MAX_LIMIT: i64 = 200;

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct SoundFile {
    /// unique file_name generated as snowflake
    // #[serde(skip)]
    pub id: SoundFileId,
    pub display_name: Option<String>,
    #[serde(skip)]
    pub owner: Option<UserId>,
    pub time_added: Option<NaiveDateTime>,
    #[serde(skip)]
    pub is_deleted: bool,
    #[serde(default)]
    pub is_public: bool,
}

impl SoundFile {
    pub fn new(id: SoundFileId, display_name: String, owner: Option<UserId>, is_public: bool) -> Self {
        return Self {
            id,
            display_name: Some(display_name),
            owner,
            time_added: None,
            is_deleted: false,
            is_public,
        };
    }

    pub async fn insert(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<(), KekServerError> {
        sqlx::query!(
            "
            INSERT INTO files (id, display_name, owner, is_public)
            VALUES ($1, $2, $3, $4)
            ",
            self.id.0 as i64,
            self.display_name,
            self.owner.as_ref().map(|o| o.0 as i64),
            self.is_public
        )
        .execute(transaction)
        .await?;
        return Ok(());
    }

    pub async fn toggle_visibility(
        id: &SoundFileId,
        owner: &UserId,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<Self>, KekServerError> {
        match sqlx::query!(
            "
            UPDATE files
            SET is_public = NOT is_public
            WHERE id = $1 AND owner = $2
            RETURNING *
            ",
            id.0 as i64,
            owner.0 as i64
        )
        .fetch_optional(transaction)
        .await?
        {
            Some(r) => {
                return Ok(Some(Self {
                    id: SoundFileId(r.id as u64),
                    owner: r.owner.map(|o| UserId(o as u64)),
                    display_name: r.display_name,
                    time_added: Some(r.time_added),
                    is_public: r.is_public.unwrap_or(false),
                    is_deleted: r.is_deleted.unwrap_or(false),
                }));
            }
            None => return Ok(None),
        }
    }

    pub async fn delete(
        id: &SoundFileId,
        owner: &UserId,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<Self>, KekServerError> {
        match sqlx::query!(
            "
            UPDATE files
            SET is_deleted = true
            WHERE id = $1 AND owner = $2
            RETURNING *
            ",
            id.0 as i64,
            owner.0 as i64
        )
        .fetch_optional(transaction)
        .await?
        {
            Some(r) => {
                return Ok(Some(Self {
                    id: SoundFileId(r.id as u64),
                    owner: r.owner.map(|o| UserId(o as u64)),
                    display_name: r.display_name,
                    time_added: Some(r.time_added),
                    is_public: r.is_public.unwrap_or(false),
                    is_deleted: r.is_deleted.unwrap_or(false),
                }));
            }
            None => return Ok(None),
        }
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
        .fetch_all(transaction)
        .await?;
        let rows_deleted = records
            .into_iter()
            .map(|r| Self {
                id: SoundFileId(r.id as u64),
                owner: r.owner.map(|o| UserId(o as u64)),
                display_name: r.display_name,
                time_added: Some(r.time_added),
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
                    time_added: Some(r.time_added),
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
                time_added: Some(r.time_added),
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
    ) -> Result<Vec<Self>, KekServerError> {
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

        let files = records
            .into_iter()
            .map(|r| Self {
                id: SoundFileId(r.id as u64),
                owner: r.owner.map(|o| UserId(o as u64)),
                display_name: r.display_name,
                time_added: Some(r.time_added),
                is_public: r.is_public.unwrap_or(true),
                is_deleted: r.is_deleted.unwrap_or(false),
            })
            .collect();
        return Ok(files);
    }
}
