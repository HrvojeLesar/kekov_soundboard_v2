use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};

use crate::{
    error::errors::KekServerError, utils::deserialize_string_to_number,
    utils::serialize_i64_to_string, utils::serialize_id_to_string,
};

use super::ids::UserId;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(deserialize_with = "deserialize_string_to_number")]
    #[serde(serialize_with = "serialize_id_to_string")]
    id: UserId,
    username: String,
    avatar: Option<String>,
}

impl User {
    pub async fn get_with_id(
        id: &UserId,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<Self>, KekServerError> {
        match sqlx::query!(
            "
            SELECT * FROM users
            WHERE id = $1
            ",
            id.0 as i64
        )
        .fetch_optional(&mut *transaction)
        .await?
        {
            Some(r) => {
                return Ok(Some(Self {
                    id: UserId(r.id as u64),
                    username: r.username,
                    avatar: r.avatar,
                }));
            }
            None => return Ok(None),
        }
    }

    pub async fn insert_user(
        id: &UserId,
        username: &String,
        avatar: Option<&String>,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<(), KekServerError> {
        sqlx::query!(
            "
            INSERT INTO users (id, username, avatar)
            VALUES ($1, $2, $3)
            ",
            id.0 as i64,
            username,
            avatar,
        )
        .execute(&mut *transaction)
        .await?;
        return Ok(());
    }

    pub fn get_id(&self) -> &UserId {
        return &self.id;
    }

    pub fn get_username(&self) -> &String {
        return &self.username;
    }

    pub fn get_avatar(&self) -> Option<&String> {
        return self.avatar.as_ref();
    }
}
