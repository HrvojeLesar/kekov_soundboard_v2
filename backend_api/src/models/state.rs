use chrono::{DateTime, Utc};
use oauth2::{url::Url, CsrfToken, PkceCodeChallenge, PkceCodeVerifier};
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
        csrf_token: &str,
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
        csrf_token: &str,
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

    pub async fn insert_state(
        transaction: &mut Transaction<'_, Postgres>,
        csrf_token: CsrfToken,
        pkce_verifier: PkceCodeVerifier,
    ) -> Result<(), KekServerError> {
        sqlx::query!(
            "
            INSERT INTO state (csrf_token, pkce_verifier)
            VALUES ($1, $2)
            ",
            csrf_token.secret(),
            pkce_verifier.secret(),
        )
        .execute(&mut *transaction)
        .await?;
        return Ok(());
    }

    pub async fn check_collision(
        transaction: &mut Transaction<'_, Postgres>,
        auth_url: &mut Url,
        csrf_token: &mut CsrfToken,
        pkce_challange: PkceCodeChallenge,
        oauth_client_url_fn: impl Fn(PkceCodeChallenge) -> (Url, CsrfToken),
    ) -> Result<(), KekServerError> {
        while sqlx::query!(
            // TODO: Expire or cleanup old states
            // Replace or update old state if key matches (csrf_token)
            "
            SELECT * FROM state
            WHERE csrf_token = $1
            ",
            csrf_token.secret()
        )
        .fetch_optional(&mut *transaction)
        .await?
        .is_some()
        {
            (*auth_url, *csrf_token) = oauth_client_url_fn(pkce_challange.clone());
        }
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
