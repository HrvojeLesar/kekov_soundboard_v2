use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};

use crate::{error::errors::KekServerError, utils::deserialize_string_to_number};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Guild {
    #[serde(deserialize_with = "deserialize_string_to_number")]
    id: i64,
    name: String,
    icon: Option<String>,
    icon_hash: Option<String>,
}

impl Guild {
    pub fn new(id: i64, name: String, icon: Option<String>, icon_hash: Option<String>) -> Self {
        return Self {
            id,
            name,
            icon,
            icon_hash,
        };
    }

    pub async fn get_guild_from_id(
        id: &i64,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<Self>, KekServerError> {
        return Ok(sqlx::query_as!(
            Self,
            "
            SELECT * FROM guild
            WHERE id = $1
            ",
            id
        )
        .fetch_optional(&mut *transaction)
        .await?);
    }

    pub async fn insert_guild(
        id: &i64,
        name: &String,
        icon: Option<&String>,
        icon_hash: Option<&String>,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<(), KekServerError> {
        sqlx::query!(
            "
            INSERT INTO guild (id, name, icon, icon_hash)
            VALUES ($1, $2, $3, $4)
            ",
            id,
            name,
            icon,
            icon_hash
        )
        .execute(&mut *transaction)
        .await?;
        return Ok(());
    }

    pub async fn get_existing_guilds(
        guilds: &Vec<Self>,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Vec<Self>, KekServerError> {
        let ids = guilds.iter().map(|guild| guild.id).collect::<Vec<i64>>();
        return Ok(sqlx::query_as!(
            Self,
            "
            SELECT * FROM guild
            WHERE id = ANY($1)
            ",
            &ids
        )
        .fetch_all(transaction)
        .await?);
    }

    pub fn get_id(&self) -> &i64 {
        return &self.id;
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }

    pub fn get_icon(&self) -> Option<&String> {
        return self.icon.as_ref();
    }

    pub fn get_icon_hash(&self) -> Option<&String> {
        return self.icon_hash.as_ref();
    }
}
