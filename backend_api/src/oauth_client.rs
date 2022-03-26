use crate::discord_client_config::DiscordClientConfig;
use oauth2::{basic::BasicClient, CsrfToken, PkceCodeChallenge, url::Url};

pub struct OAuthClient {
    client: BasicClient,
    config: DiscordClientConfig,
}

impl OAuthClient {
    pub fn new() -> Self {
        let config = DiscordClientConfig::new();
        return Self {
            client: BasicClient::new(
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
        return self.client
            .authorize_url(CsrfToken::new_random)
            .set_pkce_challenge(pkce_challange)
            .add_scopes(self.config.scopes.clone())
            .url();
    }

    pub fn get_client(&self) -> &BasicClient {
        return &self.client;
    }
}
