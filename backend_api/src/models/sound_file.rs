use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};

use crate::error::errors::KekServerError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SoundFile {
    /// unique file_name generated as snowflake
    // #[serde(skip)]
    pub id: i64,
    pub display_name: Option<String>,
    #[serde(skip)]
    pub owner: Option<i64>,
}

impl SoundFile {
    pub fn new(id: i64, display_name: String, owner: i64) -> Self {
        return Self {
            id,
            display_name: Some(display_name),
            owner: Some(owner),
        };
    }

    pub fn get_id(&self) -> &i64 {
        return &self.id;
    }

    pub fn get_display_name(&self) -> Option<&String> {
        return self.display_name.as_ref();
    }

    pub fn get_owner(&self) -> Option<&i64> {
        return self.owner.as_ref();
    }

    pub async fn insert(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<(), KekServerError> {
        let owner = match self.get_owner() {
            Some(o) => Some(*o),
            None => None,
        };

        sqlx::query!(
            "
            INSERT INTO files (id, display_name, owner)
            VALUES ($1, $2, $3)
            ",
            self.get_id(),
            self.get_display_name(),
            owner
        )
        .execute(transaction)
        .await?;
        return Ok(());
    }

    pub async fn delete_static(
        id: i64,
        owner: i64,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<Self>, KekServerError> {
        let rows_deleted = sqlx::query_as!(
            Self,
            "
            DELETE FROM files
            WHERE id = $1 AND owner = $2
            RETURNING *
            ",
            id,
            owner
        )
        .fetch_optional(transaction)
        .await?;
        return Ok(rows_deleted);
    }

    pub async fn delete_multiple_static(
        ids: &Vec<i64>,
        owner: i64,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Vec<Self>, KekServerError> {
        let rows_deleted = sqlx::query_as!(
            Self,
            "
            DELETE FROM files
            WHERE id = ANY($1) AND owner = $2
            RETURNING *
            ",
            ids,
            owner
        )
        .fetch_all(transaction)
        .await?;
        return Ok(rows_deleted);
    }

    pub async fn get_file_from_id(
        id: &i64,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<Self>, KekServerError> {
        return Ok(sqlx::query_as!(
            Self,
            "
            SELECT * FROM files
            WHERE id = $1
            ",
            id
        )
        .fetch_optional(&mut *transaction)
        .await?);
    }
}
