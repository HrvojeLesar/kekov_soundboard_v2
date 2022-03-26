use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, RevocationUrl, Scope, TokenUrl};
use std::env;

pub struct DiscordClientConfig {
    pub client_id: ClientId,
    pub client_secret: ClientSecret,
    pub scopes: Vec<Scope>,
    pub redirect_url: RedirectUrl,
    pub auth_url: AuthUrl,
    pub token_url: TokenUrl,
    pub revocation_url: RevocationUrl,
}

impl DiscordClientConfig {
    pub fn new() -> Self {
        Self {
            client_id: ClientId::new(
                env::var("DISCORD_CLIENT_ID").expect("DISCORD_CLIENT_ID must be set!"),
            ),
            client_secret: ClientSecret::new(
                env::var("DISCORD_CLIENT_SECRET").expect("DISCORD_CLIENT_SECRET must be set!"),
            ),
            scopes: vec![
                Scope::new("identify".to_string()),
                Scope::new("email".to_string()),
            ],
            redirect_url: RedirectUrl::new("https://localhost:8080/authcallback".to_string())
                .expect("Requires valid redirect url!"),
            auth_url: AuthUrl::new("https://discord.com/api/oauth2/authorize".to_string())
                .expect("Required valid auth url!"),
            token_url: TokenUrl::new("https://discord.com/api/oauth2/token".to_string())
                .expect("Required valid token url!"),
            revocation_url: RevocationUrl::new(
                "https://discord.com/api/oauth2/token/revoke".to_string(),
            )
            .expect("Required valid token revocation url!"),
        }
    }
}
