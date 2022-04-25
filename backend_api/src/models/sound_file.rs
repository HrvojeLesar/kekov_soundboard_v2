use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};

use crate::error::errors::KekServerError;

use super::ids::{SoundFileId, UserId};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SoundFile {
    /// unique file_name generated as snowflake
    // #[serde(skip)]
    pub id: SoundFileId,
    pub display_name: Option<String>,
    #[serde(skip)]
    pub owner: Option<UserId>,
}

impl SoundFile {
    pub fn new(id: SoundFileId, display_name: String, owner: UserId) -> Self {
        return Self {
            id,
            display_name: Some(display_name),
            owner: Some(owner),
        };
    }

    pub fn get_id(&self) -> &SoundFileId {
        return &self.id;
    }

    pub fn get_display_name(&self) -> Option<&String> {
        return self.display_name.as_ref();
    }

    pub fn get_owner(&self) -> Option<&UserId> {
        return self.owner.as_ref();
    }

    pub async fn insert(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<(), KekServerError> {
        let owner = match self.get_owner() {
            Some(o) => Some(o.0 as i64),
            None => None,
        };

        sqlx::query!(
            "
            INSERT INTO files (id, display_name, owner)
            VALUES ($1, $2, $3)
            ",
            self.get_id().0 as i64,
            self.get_display_name(),
            owner
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
            DELETE FROM files
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
                }));
            }
            None => return Ok(None),
        }
    }

    pub async fn delete_multiple_static(
        ids: &Vec<SoundFileId>,
        owner: &UserId,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Vec<Self>, KekServerError> {
        let ids = ids.iter().map(|id| id.0 as i64).collect::<Vec<i64>>();
        let records = sqlx::query!(
            "
            DELETE FROM files
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
            WHERE owner = $1
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
                }
            })
            .collect();
        return Ok(files);
    }
}
