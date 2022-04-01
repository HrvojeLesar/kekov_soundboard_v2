use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};

use crate::error::errors::KekServerError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SoundFile {
    /// unique file_name generated as snowflake
    // #[serde(skip)]
    id: i64,
    display_name: String,
    #[serde(skip)]
    owner: Option<u64>,
}

impl SoundFile {
    pub fn new(id: i64, display_name: String, owner: u64) -> Self {
        return Self {
            id,
            display_name,
            owner: Some(owner),
        };
    }

    pub fn get_id(&self) -> &i64 {
        return &self.id;
    }

    pub fn get_display_name(&self) -> &String {
        return &self.display_name;
    }

    pub fn get_owner(&self) -> Option<&u64> {
        return self.owner.as_ref();
    }

    pub async fn insert(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<(), KekServerError> {
        let owner = match self.get_owner() {
            Some(o) => Some(*o as i64),
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
}
