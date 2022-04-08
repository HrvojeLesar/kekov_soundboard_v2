use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};

use crate::{
    error::errors::KekServerError, utils::deserialize_string_to_number,
    utils::serialize_i64_to_string,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(deserialize_with = "deserialize_string_to_number")]
    #[serde(serialize_with = "serialize_i64_to_string")]
    id: i64,
    username: String,
    avatar: Option<String>,
}

impl User {
    pub async fn get_with_id(
        id: &i64,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<Self>, KekServerError> {
        return Ok(sqlx::query_as!(
            Self,
            "
            SELECT * FROM users
            WHERE id = $1
            ",
            id
        )
        .fetch_optional(&mut *transaction)
        .await?);
    }

    pub async fn insert_user(
        id: &i64,
        username: &String,
        avatar: Option<&String>,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<(), KekServerError> {
        sqlx::query!(
            "
            INSERT INTO users (id, username, avatar)
            VALUES ($1, $2, $3)
            ",
            id,
            username,
            avatar,
        )
        .execute(&mut *transaction)
        .await?;
        return Ok(());
    }

    pub fn get_id(&self) -> &i64 {
        return &self.id;
    }

    pub fn get_username(&self) -> &String {
        return &self.username;
    }

    pub fn get_avatar(&self) -> Option<&String> {
        return self.avatar.as_ref();
    }
}
