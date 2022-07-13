use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};

use crate::{error::errors::KekServerError, utils::cache::DiscordGuild};

use super::ids::GuildId;

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Guild {
    pub id: GuildId,
    pub name: String,
    pub time_added: NaiveDateTime,
    pub active: bool,
}

impl Guild {
    pub async fn get_guild_from_id(
        id: &GuildId,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<Self>, KekServerError> {
        match sqlx::query!(
            "
            SELECT * FROM guild
            WHERE id = $1 AND active = true
            ",
            id.0 as i64
        )
        .fetch_optional(&mut *transaction)
        .await?
        {
            Some(r) => {
                return Ok(Some(Self {
                    id: r.id.into(),
                    name: r.name,
                    time_added: r.time_added,
                    active: r.active,
                }));
            }
            None => return Ok(None),
        }
    }

    pub async fn insert_guild(
        id: &GuildId,
        name: &str,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Self, KekServerError> {
        let r = sqlx::query!(
            "
            INSERT INTO guild (id, name)
            VALUES ($1, $2)
            ON CONFLICT (id)
            DO UPDATE
            SET active = true
            RETURNING *
            ",
            id.0 as i64,
            name
        )
        .fetch_one(&mut *transaction)
        .await?;

        return Ok(Self {
            id: r.id.into(),
            name: r.name,
            time_added: r.time_added,
            active: r.active,
        });
    }

    pub async fn get_intercepting_user_and_bot_guilds(
        guilds: &[DiscordGuild],
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Vec<Self>, KekServerError> {
        let ids = guilds
            .iter()
            .map(|guild| guild.id.0 as i64)
            .collect::<Vec<i64>>();
        let records = sqlx::query!(
            "
            SELECT * FROM guild
            WHERE id = ANY($1) AND active = true
            ",
            &ids
        )
        .fetch_all(&mut *transaction)
        .await?;
        let guilds = records
            .into_iter()
            .map(|r| Guild {
                id: r.id.into(),
                name: r.name,
                time_added: r.time_added,
                active: r.active,
            })
            .collect::<Vec<Self>>();
        return Ok(guilds);
    }

    pub async fn remove_guild(
        id: &GuildId,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<Self>, KekServerError> {
        match sqlx::query!(
            "
            UPDATE guild
            SET active = false
            WHERE id = $1 AND active = true
            RETURNING *
            ",
            id.0 as i64
        )
        .fetch_optional(&mut *transaction)
        .await?
        {
            Some(r) => {
                return Ok(Some(Self {
                    id: r.id.into(),
                    name: r.name,
                    time_added: r.time_added,
                    active: r.active,
                }));
            }
            None => return Ok(None),
        }
    }

    pub async fn get_all_guilds(
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Vec<Self>, KekServerError> {
        return Ok(sqlx::query!(
            "
            SELECT * FROM guild
            ORDER BY id
            "
        )
        .fetch_all(&mut *transaction)
        .await?
        .into_iter()
        .map(|r| Guild {
            id: r.id.into(),
            name: r.name,
            time_added: r.time_added,
            active: r.active,
        })
        .collect());
    }
}

#[cfg(test)]
mod tests {
    use sqlx::Connection;
    use uuid::Uuid;

    use crate::{
        database::tests_db_helper::db_connection,
        models::ids::GuildId,
        utils::{
            cache::DiscordGuild,
            test_utils::{self, insert_guild_test_util},
        },
    };

    use super::Guild;

    #[actix_web::test]
    async fn test_get_guild_from_id() {
        let mut connection = db_connection().await;
        let mut transaction = connection.begin().await.unwrap();
        let inserted_guild = test_utils::insert_guild_test_util(&mut transaction).await;

        let guild = Guild::get_guild_from_id(&inserted_guild.id, &mut transaction)
            .await
            .unwrap()
            .unwrap();
        transaction.commit().await.unwrap();

        assert_eq!(guild.id, inserted_guild.id);
        assert_eq!(
            guild.time_added.timestamp(),
            inserted_guild.time_added.timestamp()
        );
        assert!(guild.active);
    }

    #[actix_web::test]
    async fn test_insert_guild() {
        let guild_id = GuildId(Uuid::new_v4().as_u128() as u64);
        let mut connection = db_connection().await;
        let mut transaction = connection.begin().await.unwrap();
        Guild::insert_guild(&guild_id, "Test", &mut transaction)
            .await
            .unwrap();
        let guild = Guild::get_guild_from_id(&guild_id, &mut transaction)
            .await
            .unwrap();
        transaction.commit().await.unwrap();
        assert!(guild.is_some());
    }

    #[actix_web::test]
    async fn test_get_intercepting_user_and_bot_guilds() {
        let mut connection = db_connection().await;
        let mut transaction = connection.begin().await.unwrap();

        let mut test_guilds = vec![];
        for _ in 0..5 {
            let guild = insert_guild_test_util(&mut transaction).await;
            let dguild = DiscordGuild {
                id: guild.id,
                name: guild.name,
                icon: None,
                icon_hash: None,
            };
            test_guilds.push(dguild);
        }

        let guilds = Guild::get_intercepting_user_and_bot_guilds(&test_guilds, &mut transaction)
            .await
            .unwrap();
        transaction.commit().await.unwrap();

        for guild in &guilds {
            assert!(test_guilds.iter().find(|tg| &tg.id == &guild.id).is_some());
        }
        assert_eq!(test_guilds.len(), guilds.len());
    }

    #[actix_web::test]
    async fn test_remove_guild() {
        let mut connection = db_connection().await;
        let mut transaction = connection.begin().await.unwrap();
        let inserted_guild = test_utils::insert_guild_test_util(&mut transaction).await;
        let guild = Guild::remove_guild(&inserted_guild.id, &mut transaction)
            .await
            .unwrap()
            .unwrap();
        transaction.commit().await.unwrap();
        assert_eq!(guild.id, inserted_guild.id);
        assert_ne!(guild.active, inserted_guild.active);
    }
}
