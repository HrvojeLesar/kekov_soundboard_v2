use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};

use crate::error::errors::KekServerError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    csrf_token: String,
    pkce_verifier: String,
    expires: DateTime<Utc>,
}

impl State {
    pub async fn get_with_token(
        csrf_token: &String,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<Self>, KekServerError> {
        return Ok(sqlx::query_as!(
            Self,
            "
            SELECT * FROM state
            WHERE csrf_token = $1
            ",
            csrf_token
        )
        .fetch_optional(&mut *transaction)
        .await?);
    }

    pub async fn delete_state(
        csrf_token: &String,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<(), KekServerError> {
        sqlx::query!(
            "
            DELETE FROM state
            WHERE csrf_token = $1
            ",
            csrf_token
        )
        .execute(&mut *transaction)
        .await?;
        return Ok(());
    }

    pub fn get_csrf_token(&self) -> &String {
        return &self.csrf_token;
    }

    pub fn get_pkce_verifier(&self) -> &String {
        return &self.pkce_verifier;
    }

    pub fn get_expires_date(&self) -> &DateTime<Utc> {
        return &self.expires;
    }
}
