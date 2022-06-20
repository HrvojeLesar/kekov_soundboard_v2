use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};

use crate::{error::errors::KekServerError, utils::cache::DiscordGuild};

use super::ids::GuildId;

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Guild {
    pub id: GuildId,
    pub name: String,
    pub icon: Option<String>,
    pub icon_hash: Option<String>,
    pub time_added: Option<NaiveDateTime>,
}

impl Guild {
    pub async fn get_guild_from_id(
        id: &GuildId,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Option<Self>, KekServerError> {
        match sqlx::query!(
            "
            SELECT * FROM guild
            WHERE id = $1
            ",
            id.0 as i64
        )
        .fetch_optional(&mut *transaction)
        .await?
        {
            Some(r) => {
                return Ok(Some(Self {
                    id: GuildId(r.id as u64),
                    name: r.name,
                    icon: r.icon,
                    icon_hash: r.icon_hash,
                    time_added: Some(r.time_added),
                }));
            }
            None => return Ok(None),
        }
    }

    pub async fn insert_guild(
        id: &GuildId,
        name: &str,
        icon: Option<&String>,
        icon_hash: Option<&String>,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<(), KekServerError> {
        sqlx::query!(
            "
            INSERT INTO guild (id, name, icon, icon_hash)
            VALUES ($1, $2, $3, $4)
            ",
            id.0 as i64,
            name,
            icon,
            icon_hash
        )
        .execute(&mut *transaction)
        .await?;
        return Ok(());
    }

    pub async fn get_existing_guilds(
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
            WHERE id = ANY($1)
            ",
            &ids
        )
        .fetch_all(transaction)
        .await?;
        let guilds = records
            .into_iter()
            .map(|r| Guild {
                id: GuildId(r.id as u64),
                name: r.name,
                icon: r.icon,
                icon_hash: r.icon_hash,
                time_added: Some(r.time_added),
            })
            .collect::<Vec<Self>>();
        return Ok(guilds);
    }

    pub fn get_id(&self) -> &GuildId {
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

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, Utc};
    use uuid::Uuid;

    use crate::{database::tests_db_helper::DB_POOL, models::ids::GuildId, utils::cache::DiscordGuild};

    use super::Guild;

    #[actix_web::test]
    async fn test_get_guild_from_id() {
        let guild_id = GuildId(Uuid::new_v4().as_u128() as u64);
        let now = Utc::now().naive_utc();
        let mut transaction = DB_POOL.begin().await.unwrap();
        sqlx::query!(
            "
            INSERT INTO guild (id, name, icon, icon_hash, time_added)
            VALUES ($1, 'Test', 'icon', 'icon_hash', $2)
            ",
            guild_id.0 as i64,
            now,
        )
        .execute(&mut transaction)
        .await
        .unwrap();

        let guild = Guild::get_guild_from_id(&guild_id, &mut transaction)
            .await
            .unwrap()
            .unwrap();
        transaction.commit().await.unwrap();

        assert_eq!(guild.id, guild_id);
        assert_eq!(guild.time_added.unwrap().timestamp(), now.timestamp());
    }

    #[actix_web::test]
    async fn test_insert_guild() {
        let guild_id = GuildId(Uuid::new_v4().as_u128() as u64);
        let mut transaction = DB_POOL.begin().await.unwrap();
        Guild::insert_guild(&guild_id, "Test", None, None, &mut transaction)
            .await
            .unwrap();
        let guild = Guild::get_guild_from_id(&guild_id, &mut transaction)
            .await
            .unwrap();
        transaction.commit().await.unwrap();
        assert!(guild.is_some());
    }

    #[actix_web::test]
    async fn test_get_existing_guilds() {
        let mut transaction = DB_POOL.begin().await.unwrap();

        let mut test_guilds = vec![];
        for _ in 0..5 {
            let guild_id = GuildId(Uuid::new_v4().as_u128() as u64);
            Guild::insert_guild(&guild_id, "Test", None, None, &mut transaction)
                .await
                .unwrap();
            test_guilds.push(DiscordGuild {
                id: guild_id,
                name: "Test".to_string(),
                icon: None,
                icon_hash: None,
            });
        }

        let guilds = Guild::get_existing_guilds(&test_guilds, &mut transaction)
            .await
            .unwrap();
        transaction.commit().await.unwrap();

        for guild in &guilds {
            assert!(test_guilds.iter().find(|tg| &tg.id == &guild.id).is_some());
        }
        assert_eq!(test_guilds.len(), guilds.len());
    }
}
