use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};

use crate::error::errors::KekServerError;

use super::ids::{SoundFileId, UserId};

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
    #[serde(skip)]
    pub is_public: bool,
}

impl SoundFile {
    pub fn new(id: SoundFileId, display_name: String, owner: UserId, is_public: Option<bool>) -> Self {
        return Self {
            id,
            display_name: Some(display_name),
            owner: Some(owner),
            time_added: None,
            is_deleted: false,
            is_public: is_public.unwrap_or(false),
        };
    }

    pub async fn insert(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<(), KekServerError> {
        let owner = match &self.owner {
            Some(o) => Some(o.0 as i64),
            None => None,
        };

        sqlx::query!(
            "
            INSERT INTO files (id, display_name, owner, is_public)
            VALUES ($1, $2, $3, $4)
            ",
            self.id.0 as i64,
            self.display_name,
            owner,
            self.is_public
        )
        .execute(transaction)
        .await?;
        return Ok(());
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
                let owner = match r.owner {
                    Some(o) => Some(UserId(o as u64)),
                    None => None,
                };
                return Ok(Some(Self {
                    id: SoundFileId(r.id as u64),
                    owner,
                    display_name: r.display_name,
                    time_added: Some(r.time_added),
                    is_public: r.is_public.unwrap_or(false),
                    is_deleted: r.is_deleted.unwrap_or(false)
                }));
            }
            None => return Ok(None),
        }
    }

    pub async fn delete_multiple(
        ids: &Vec<SoundFileId>,
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
            .map(|r| {
                let owner = match r.owner {
                    Some(o) => Some(UserId(o as u64)),
                    None => None,
                };
                Self {
                    id: SoundFileId(r.id as u64),
                    owner,
                    display_name: r.display_name,
                    time_added: Some(r.time_added),
                    is_public: r.is_public.unwrap_or(false),
                    is_deleted: r.is_deleted.unwrap_or(false)
                }
            })
            .collect::<Vec<SoundFile>>();
        return Ok(rows_deleted);
    }

    pub async fn get_file_from_id(
        id: &SoundFileId,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<Self>, KekServerError> {
        match sqlx::query!(
            "
            SELECT * FROM files
            WHERE id = $1
            ",
            id.0 as i64
        )
        .fetch_optional(&mut *transaction)
        .await?
        {
            Some(r) => {
                let owner = match r.owner {
                    Some(o) => Some(UserId(o as u64)),
                    None => None,
                };
                return Ok(Some(Self {
                    id: SoundFileId(r.id as u64),
                    owner,
                    display_name: r.display_name,
                    time_added: Some(r.time_added),
                    is_public: r.is_public.unwrap_or(false),
                    is_deleted: r.is_deleted.unwrap_or(false)
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
            .map(|r| {
                let owner = match r.owner {
                    Some(o) => Some(UserId(o as u64)),
                    None => None,
                };
                Self {
                    id: SoundFileId(r.id as u64),
                    owner,
                    display_name: r.display_name,
                    time_added: Some(r.time_added),
                    is_public: r.is_public.unwrap_or(false),
                    is_deleted: r.is_deleted.unwrap_or(false)
                }
            })
            .collect();
        return Ok(files);
    }
}
