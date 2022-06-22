use std::{str::FromStr, time::Duration};

use actix::clock::sleep;
use actix_http::{encoding::Decoder, Payload, StatusCode};
use actix_web::http::header::AUTHORIZATION;
use awc::{error::SendRequestError, ClientResponse};
use log::{debug, warn};
use serde::{Deserialize, Deserializer, Serializer};

use crate::{error::errors::KekServerError, models::ids::Id};

use self::auth::AuthorizedUser;

pub mod auth;
pub mod cache;
pub mod validation;

pub const USERGUILDS: &str = "/users/@me/guilds";
pub const MAX_RETRIES: u8 = 3;

pub fn deserialize_string_to_number<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + Deserialize<'de>,
    <T as FromStr>::Err: std::fmt::Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNum<T> {
        String(String),
        Number(T),
    }

    return match StringOrNum::<T>::deserialize(deserializer)? {
        StringOrNum::String(s) => s.parse::<T>().map_err(serde::de::Error::custom),
        StringOrNum::Number(n) => Ok(n),
    };
}

pub fn serialize_id_to_string<S>(num: &dyn Id, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    return serializer.serialize_str(&num.get_id().to_string());
}

async fn get_request(
    autorized_user: &AuthorizedUser,
    url: &str,
) -> Result<ClientResponse<Decoder<Payload>>, SendRequestError> {
    return awc::Client::new()
        .get(format!("https://discord.com/api/v9{}", url))
        .append_header((
            AUTHORIZATION,
            format!("Bearer {}", &autorized_user.access_token.0),
        ))
        .send()
        .await;
}

pub async fn make_discord_get_request(
    autorized_user: &AuthorizedUser,
    url: &str,
) -> Result<ClientResponse<Decoder<Payload>>, KekServerError> {
    let mut resp = get_request(autorized_user, url).await?;
    let mut retries = 0;
    while resp.status() == StatusCode::TOO_MANY_REQUESTS {
        warn!("Rate limit exceeded");
        if let Some(after) = resp.headers().get("retry-after") {
            let after = after.to_str()?.parse()?;
            let sleep_dur = Duration::from_secs_f64(after);
            if sleep_dur > Duration::from_secs(10) {
                return Err(KekServerError::AuthorizationTimeExpiredError);
            }
            sleep(sleep_dur).await;
        } else {
            return Err(KekServerError::DiscordRequestError);
        }
        resp = get_request(autorized_user, url).await?;

        retries += 1;
        if retries > MAX_RETRIES {
            warn!("Maximum retries exceeded!");
            break;
        }
    }

    if resp.status().is_client_error() {
        debug!("{:#?}", &resp.status());
        debug!("{:#?}", &resp.headers());
        return Err(KekServerError::DiscordRequestError);
    }

    return Ok(resp);
}

#[cfg(test)]
pub mod test_utils {
    use chrono::Utc;
    use sqlx::{Postgres, Transaction};
    use uuid::Uuid;

    use crate::{
        database::tests_db_helper::db_connection,
        models::{
            guild::Guild,
            guild_file::GuildFile,
            ids::{GuildId, SoundFileId, UserId},
            sound_file::{self, SoundFile},
            user::User,
        },
    };

    pub struct PublicDeleted {
        pub is_public: bool,
        pub is_deleted: bool,
    }

    pub async fn insert_user_test_util(transaction: &mut Transaction<'_, Postgres>) -> User {
        let user_id = UserId(Uuid::new_v4().as_u128() as u64);
        let now = Utc::now().naive_utc();
        let username = format!("Test user {}", user_id.0.clone());
        let user = User {
            id: user_id,
            username,
            avatar: None,
        };

        sqlx::query!(
            "
            INSERT INTO users (id, username)
            VALUES ($1, $2)
            ",
            user.id.0 as i64,
            user.username,
        )
        .execute(&mut *transaction)
        .await
        .unwrap();

        return user;
    }

    pub async fn insert_guild_test_util(transaction: &mut Transaction<'_, Postgres>) -> Guild {
        let guild_id = GuildId(Uuid::new_v4().as_u128() as u64);
        let now = Utc::now().naive_utc();
        let name = format!("Test guild {}", guild_id.0.clone());
        let guild = Guild {
            id: guild_id,
            name,
            time_added: now,
            icon: None,
            icon_hash: None,
        };

        sqlx::query!(
            "
            INSERT INTO guild (id, name, time_added)
            VALUES ($1, $2, $3)
            ",
            guild.id.0 as i64,
            guild.name,
            guild.time_added
        )
        .execute(&mut *transaction)
        .await
        .unwrap();

        return guild;
    }

    pub async fn insert_random_file_test_util(
        transaction: &mut Transaction<'_, Postgres>,
    ) -> SoundFile {
        let id = SoundFileId(Uuid::new_v4().as_u128() as u64);
        let owner = insert_user_test_util(&mut *transaction).await;
        let now = Utc::now().naive_utc();
        let sound_file = SoundFile {
            id,
            display_name: Some("Test file name".to_string()),
            time_added: now,
            is_deleted: false,
            is_public: false,
            owner: Some(owner.id),
        };
        sqlx::query!(
            "
            INSERT INTO files (id, display_name, owner, is_public, time_added)
            VALUES ($1, $2, $3, $4, $5)
            ",
            sound_file.id.0 as i64,
            sound_file.display_name,
            sound_file.owner.as_ref().map(|o| o.0 as i64),
            sound_file.is_public,
            sound_file.time_added
        )
        .execute(transaction)
        .await
        .unwrap();

        return sound_file;
    }

    pub async fn insert_file_test_util(
        owner_id: &UserId,
        public_deleted: Option<PublicDeleted>,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> SoundFile {
        let id = SoundFileId(Uuid::new_v4().as_u128() as u64);
        let now = Utc::now().naive_utc();
        let public_deleted = public_deleted.unwrap_or(PublicDeleted {
            is_deleted: false,
            is_public: false,
        });
        let sound_file = SoundFile {
            id,
            display_name: Some("Test file name".to_string()),
            time_added: now,
            is_deleted: public_deleted.is_deleted,
            is_public: public_deleted.is_public,
            owner: Some(owner_id.clone()),
        };
        sqlx::query!(
            "
            INSERT INTO files (id, display_name, owner, is_public, time_added, is_deleted)
            VALUES ($1, $2, $3, $4, $5, $6)
            ",
            sound_file.id.0 as i64,
            sound_file.display_name,
            sound_file.owner.as_ref().map(|o| o.0 as i64),
            sound_file.is_public,
            sound_file.time_added,
            sound_file.is_deleted
        )
        .execute(transaction)
        .await
        .unwrap();

        return sound_file;
    }

    pub async fn insert_random_guild_file_test_util(
        transaction: &mut Transaction<'_, Postgres>,
    ) -> GuildFile {
        let guild = insert_guild_test_util(&mut *transaction).await;
        let file = insert_random_file_test_util(&mut *transaction).await;
        let file_id = file.id.clone();
        let now = Utc::now().naive_utc();
        let guild_file = GuildFile {
            guild_id: guild.id,
            file_id,
            time_added: now,
            is_deleted: false,
            sound_file: Some(file),
        };

        sqlx::query!(
            "
            INSERT INTO guild_file (guild_id, file_id, time_added)
            VALUES ($1, $2, $3)
            ON CONFLICT (guild_id, file_id)
            DO UPDATE
            SET is_deleted = false;
            ",
            guild_file.guild_id.0 as i64,
            guild_file.file_id.0 as i64,
            guild_file.time_added
        )
        .execute(&mut *transaction)
        .await
        .unwrap();

        return guild_file;
    }

    pub async fn insert_guild_file_test_util(
        guild_id: &GuildId,
        sound_file: SoundFile,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> GuildFile {
        let now = Utc::now().naive_utc();
        let file_id = sound_file.id.clone();
        let guild_file = GuildFile {
            guild_id: guild_id.clone(),
            file_id,
            time_added: now,
            is_deleted: false,
            sound_file: Some(sound_file),
        };

        sqlx::query!(
            "
            INSERT INTO guild_file (guild_id, file_id, time_added)
            VALUES ($1, $2, $3)
            ON CONFLICT (guild_id, file_id)
            DO UPDATE
            SET is_deleted = false;
            ",
            guild_file.guild_id.0 as i64,
            guild_file.file_id.0 as i64,
            guild_file.time_added
        )
        .execute(&mut *transaction)
        .await
        .unwrap();

        return guild_file;
    }
}
