use std::{collections::HashSet, env};

use async_trait::async_trait;
use rocket::{
    fairing::{Fairing, Info, Kind},
    outcome::IntoOutcome,
    request::FromRequest,
};
use tracing::info;

use crate::api;

pub struct LoggedInUser(usize);

pub(crate) const USER_COOKIE: &str = "user_id";

#[async_trait]
impl<'r> FromRequest<'r> for LoggedInUser {
    type Error = crate::Error;

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        request
            .cookies()
            .get_private_pending(USER_COOKIE)
            .and_then(|it| {
                let cookie = it.value().parse().ok();
                info!("Cookie: {:?}", cookie);
                cookie
            })
            .map(LoggedInUser)
            .or_forward(())
    }
}

pub struct AuthFairing;
#[async_trait]
impl Fairing for AuthFairing {
    fn info(&self) -> rocket::fairing::Info {
        Info {
            name: "AuthFairing",
            kind: Kind::Request,
        }
    }
    async fn on_request(&self, req: &mut rocket::Request<'_>, _data: &mut rocket::Data<'_>) {
        if req.cookies().get_private(USER_COOKIE).is_none()
            && !req.uri().path().starts_with("/auth")
        {
            info!("Redirecting to auth endpoint");
            req.set_uri(rocket::uri!(api::auth));
        }
    }
}

#[derive(Clone)]
pub struct Login {
    allowed_emails: AllowedEmails,
}

#[derive(Clone)]
enum AllowedEmails {
    All,
    Some(HashSet<String>),
}

impl Login {
    pub fn new() -> eyre::Result<Self> {
        let allowed_emails = env::var("ALLOWED_EMAILS")?;
        let allowed_emails = if allowed_emails == "*" {
            AllowedEmails::All
        } else {
            AllowedEmails::Some(
                allowed_emails
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
            )
        };

        Ok(Login { allowed_emails })
    }
    pub fn is_allowed(&self, email: &str) -> bool {
        match &self.allowed_emails {
            AllowedEmails::All => true,
            AllowedEmails::Some(allowed) => allowed.contains(email),
        }
    }
}
