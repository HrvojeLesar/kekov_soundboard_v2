use std::time::Duration;

use actix::clock::sleep;
use actix_web::web::Data;
use log::info;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client, Response,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::{
    error::errors::KekServerError,
    models::{guild::Guild, ids::GuildId},
};

#[derive(Debug, Serialize, Deserialize)]
struct PartialGuild {
    id: GuildId,
    name: String,
}

#[derive(Serialize)]
struct GuildQueryParams {
    after: Option<String>,
}

pub struct ActiveGuildsCheck {
    client: Client,
    pg_pool: Data<Pool<Postgres>>,
    query_params: Option<GuildQueryParams>,
    sleep_timer: Option<Duration>,
}

impl ActiveGuildsCheck {
    pub fn new(pg_pool: Data<Pool<Postgres>>) -> Result<Self, KekServerError> {
        let token = dotenv::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN is not set!");
        let mut headers = HeaderMap::new();
        let mut token_header = HeaderValue::from_str(&format!("Bot {}", token))?;
        token_header.set_sensitive(true);
        headers.insert(AUTHORIZATION, token_header);

        let client = Client::builder().default_headers(headers).build()?;

        return Ok(Self {
            client,
            pg_pool,
            query_params: None,
            sleep_timer: None,
        });
    }

    pub async fn start(&mut self) -> Result<(), KekServerError> {
        let discord_guild_ids = self.fetch_guilds_from_discord().await?;

        let mut transaction = self.pg_pool.begin().await?;
        let db_guilds = Guild::get_all_guilds(&mut transaction).await?;
        transaction.commit().await?;

        self.check(discord_guild_ids, db_guilds).await?;

        return Ok(());
    }

    async fn fetch_guilds_from_discord(&mut self) -> Result<Vec<PartialGuild>, KekServerError> {
        let mut discord_guilds: Vec<PartialGuild> = Vec::with_capacity(100_000);
        loop {
            if let Some(sleep_for) = self.sleep_timer {
                sleep(sleep_for).await;
            }

            let response = if let Some(params) = &self.query_params {
                self.client
                    .get("https://discord.com/api/v10/users/@me/guilds")
                    .query(params)
                    .send()
                    .await?
            } else {
                self.client
                    .get("https://discord.com/api/v10/users/@me/guilds")
                    .send()
                    .await?
            };

            let rate_limit_remaining =
                if let Some(rate_limit) = response.headers().get("x-ratelimit-remaining") {
                    rate_limit.to_str()?.parse::<i64>()?
                } else {
                    0
                };

            if rate_limit_remaining == 0 {
                self.try_set_rate_limit_time(&response, "x-ratelimit-reset-after")?;
            }

            self.try_set_rate_limit_time(&response, "retry-after")?;

            let mut guilds: Vec<PartialGuild> = response.json().await?;

            let guilds_len = guilds.len();
            discord_guilds.append(&mut guilds);

            if guilds_len >= 200 {
                let last = match discord_guilds.last() {
                    Some(l) => l.id.0.to_string(),
                    None => u64::MAX.to_string(),
                };
                self.query_params = Some(GuildQueryParams { after: Some(last) });
            } else {
                break;
            }
        }

        return Ok(discord_guilds);
    }

    fn try_set_rate_limit_time(
        &mut self,
        response: &Response,
        header_name: &str,
    ) -> Result<(), KekServerError> {
        if let Some(retry_after) = response.headers().get(header_name) {
            let retry_after = retry_after.to_str()?.parse::<f64>()?;
            self.sleep_timer = Some(Duration::from_secs_f64(retry_after));
        }
        return Ok(());
    }

    async fn check(
        &self,
        discord_guilds: Vec<PartialGuild>,
        db_guilds: Vec<Guild>,
    ) -> Result<(), KekServerError> {
        let mut inserted_count = 0;
        let mut removed_count = 0;

        let mut transaction = self.pg_pool.begin().await?;
        for PartialGuild { id, name } in &discord_guilds {
            match db_guilds.binary_search_by(|g| g.id.0.cmp(&id.0)) {
                Ok(idx) => {
                    let guild = &db_guilds[idx];
                    if !guild.active {
                        Guild::insert_guild(&guild.id, &name, &mut transaction).await?;
                        inserted_count += 1;
                    }
                }
                Err(_) => {
                    Guild::insert_guild(&id, &name, &mut transaction).await?;
                    inserted_count += 1;
                }
            }
        }
        for Guild { id, active, .. } in &db_guilds {
            if !*active {
                continue;
            }
            match discord_guilds.binary_search_by(|g| g.id.0.cmp(&id.0)) {
                Ok(_) => {}
                Err(_) => {
                    Guild::remove_guild(id, &mut transaction).await?;
                    removed_count += 1;
                }
            }
        }
        transaction.commit().await?;

        info!("Inserted [{}] new guilds", inserted_count);
        info!("Removed [{}] guilds", removed_count);

        return Ok(());
    }
}
