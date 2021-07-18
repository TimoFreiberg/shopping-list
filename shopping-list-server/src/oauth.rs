use std::env;

use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    RevocationUrl, Scope, TokenUrl,
};

pub struct OAuthClient {
    client: BasicClient,
}

impl OAuthClient {
    pub async fn login(&self) {
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

        // Generate the authorization URL to which we'll redirect the user.
        let (authorize_url, csrf_state) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/userinfo.email".to_string(),
            ))
            .set_pkce_challenge(pkce_code_challenge)
            .url();
    }
}

impl OAuthClient {
    pub async fn new() -> eyre::Result<Self> {
        let client_id = ClientId::new(env::var("GOOGLE_CLIENT_ID")?);
        let client_secret = ClientSecret::new(env::var("GOOGLE_CLIENT_SECRET")?);

        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
            .expect("Invalid authorization endpoint URL");
        let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
            .expect("Invalid token endpoint URL");

        let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
            .set_redirect_uri(
                RedirectUrl::new(
                    env::var("OAUTH_REDIRECT_URL").unwrap_or("http::/localhost:8000".to_string()),
                )
                .expect("Indalid redirect URL"),
            )
            .set_revocation_uri(
                RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
                    .expect("Invalid revocation endpoint URL"),
            );

        Ok(Self { client })
    }
}
