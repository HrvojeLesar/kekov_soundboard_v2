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
        guild_ids: &Vec<DiscordGuild>,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<Vec<Self>, KekServerError> {
        let ids = guild_ids
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
