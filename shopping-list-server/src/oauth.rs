use std::{
    collections::HashMap,
    env,
    sync::{Arc, Mutex},
};

use eyre::{eyre, Context};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, url::Url, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RevocationUrl,
    Scope, TokenResponse, TokenUrl,
};
use reqwest::Client;
use serde::Deserialize;
use tracing::{info, warn};

pub struct AuthRequestQuery {
    code: AuthorizationCode,
    state: CsrfToken,
}

impl AuthRequestQuery {
    pub fn new(code: AuthorizationCode, state: CsrfToken) -> Self {
        Self { code, state }
    }
}

pub struct OAuthClient {
    oauth_client: BasicClient,
    http_client: Client,
}

#[derive(Default)]
pub struct Challenges(Arc<Mutex<HashMap<String, PkceCodeVerifier>>>);

#[derive(Deserialize)]
struct UserInfo {
    email: String,
}

impl OAuthClient {
    pub async fn new() -> eyre::Result<Self> {
        let client_id = ClientId::new(env::var("GOOGLE_CLIENT_ID").context("GOOGLE_CLIENT_ID")?);
        let client_secret =
            ClientSecret::new(env::var("GOOGLE_CLIENT_SECRET").context("GOOGLE_CLIENT_SECRET")?);

        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
            .expect("Invalid authorization endpoint URL");
        let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
            .expect("Invalid token endpoint URL");

        let redirect_host =
            env::var("OAUTH_REDIRECT_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
        let redirect_url =
            RedirectUrl::new(redirect_host + "/auth/authorized").expect("Invalid redirect URL");
        let oauth_client =
            BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
                .set_redirect_uri(redirect_url)
                .set_revocation_uri(
                    RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
                        .expect("Invalid revocation endpoint URL"),
                );

        let http_client = reqwest::Client::new();

        Ok(Self {
            oauth_client,
            http_client,
        })
    }
    #[tracing::instrument(skip(self, challenges))]
    pub async fn login(&self, challenges: &Challenges) -> Url {
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

        // Generate the authorization URL to which we'll redirect the user.
        let (authorize_url, csrf_state) = self
            .oauth_client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/userinfo.email".to_string(),
            ))
            .set_pkce_challenge(pkce_code_challenge)
            .url();
        info!("Redirecting to {:?}", authorize_url);
        let csrf_secret = csrf_state.secret().to_string();
        let existing_challenge = challenges
            .0
            .lock()
            .unwrap()
            .insert(csrf_secret, pkce_code_verifier);
        if let Some(_existing) = existing_challenge {
            warn!("Existing challenge overwritten");
        };
        authorize_url
    }

    #[tracing::instrument(skip(self, auth_request_query, challenges), err)]
    pub async fn verify_challenge(
        &self,
        auth_request_query: AuthRequestQuery,
        challenges: &Challenges,
    ) -> eyre::Result<String> {
        let pkce_code_verifier = challenges
            .0
            .lock()
            .unwrap()
            .remove(auth_request_query.state.secret())
            .ok_or_else(|| eyre!("Unknown query code"))?;

        let token = self
            .oauth_client
            .exchange_code(auth_request_query.code)
            .set_pkce_verifier(pkce_code_verifier)
            .request_async(async_http_client)
            .await
            .context("Failed to exchange auth code")?;

        let user_info: UserInfo = self
            .http_client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .bearer_auth(token.access_token().secret())
            .send()
            .await?
            .json()
            .await?;

        Ok(user_info.email)
    }
}
