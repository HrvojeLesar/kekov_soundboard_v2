use crate::{discord_client_config::DiscordClientConfig, utils::cache::DiscordGuild};
use oauth2::{
    basic::{
        BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
        BasicTokenType,
    },
    url::Url,
    Client, CsrfToken, ExtraTokenFields, PkceCodeChallenge, StandardRevocableToken,
    StandardTokenResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildTokenField {
    pub guild: Option<DiscordGuild>,
}

impl ExtraTokenFields for GuildTokenField {}

type CustomOAuthClient = Client<
    BasicErrorResponse,
    StandardTokenResponse<GuildTokenField, BasicTokenType>,
    BasicTokenType,
    BasicTokenIntrospectionResponse,
    StandardRevocableToken,
    BasicRevocationErrorResponse,
>;

pub struct OAuthClient {
    client: CustomOAuthClient,
    config: DiscordClientConfig,
}

impl OAuthClient {
    pub fn new() -> Self {
        let config = DiscordClientConfig::new();
        return Self {
            client: CustomOAuthClient::new(
                config.client_id.clone(),
                Some(config.client_secret.clone()),
                config.auth_url.clone(),
                Some(config.token_url.clone()),
            )
            // .set_redirect_uri(config.redirect_url.clone())
            .set_revocation_uri(config.revocation_url.clone()),
            config,
        };
    }

    pub fn get_url(&self, pkce_challange: PkceCodeChallenge) -> (Url, CsrfToken) {
        return self
            .client
            .authorize_url(CsrfToken::new_random)
            .set_pkce_challenge(pkce_challange)
            .add_scopes(self.config.scopes.clone())
            .url();
    }

    pub fn get_bot_url(&self, pkce_challange: PkceCodeChallenge) -> (Url, CsrfToken) {
        return self
            .client
            .authorize_url(CsrfToken::new_random)
            .set_pkce_challenge(pkce_challange)
            .add_scopes(self.config.bot_scopes.clone())
            .add_extra_param("permissions", "3147776")
            .url();
    }

    pub fn get_client(&self) -> &CustomOAuthClient {
        return &self.client;
    }
}
