use chrono::Utc;
use oauth2::{AuthorizationCode, CsrfToken};
use rocket::{
    get,
    http::{Cookie, CookieJar, SameSite},
    post, put,
    response::{status::Unauthorized, Redirect},
    serde::json::Json,
    State,
};
use serde::Deserialize;
use tracing::{info, warn};

use crate::{
    login::{LoggedInUser, Login, USER_COOKIE},
    model::{ItemId, Items},
    oauth::{AuthRequestQuery, Challenges, OAuthClient},
    repo::Repository,
    OpenItem, Result,
};

#[tracing::instrument(skip(repo, _logged_in), err)]
#[get("/items?<offset>&<limit>&<show_done_items>")]
pub async fn get_items(
    repo: &State<Repository>,
    offset: Option<usize>,
    limit: Option<usize>,
    show_done_items: bool,
    _logged_in: LoggedInUser,
) -> Result<Json<Items>> {
    let items = repo.get_items(offset, limit, show_done_items).await?;
    Ok(Json(items))
}

#[tracing::instrument(skip(repo, _logged_in), err)]
#[post("/items?<show_done_items>", format = "json", data = "<body>")]
pub async fn add_item(
    repo: &State<Repository>,
    body: Json<AddItemBody>,
    show_done_items: bool,
    _logged_in: LoggedInUser,
) -> Result<Json<Items>> {
    let now = Utc::now();
    let item = OpenItem {
        id: ItemId::default(),
        name: body.into_inner().name,
        created_at: now,
    };
    repo.add_open_item(item).await?;
    let items = repo.get_items(None, None, show_done_items).await?;
    Ok(Json(items))
}

#[derive(Deserialize, Debug)]
pub struct AddItemBody {
    name: String,
}

#[tracing::instrument(skip(repo, _logged_in), err)]
#[put("/items/<id>/complete?<show_done_items>")]
pub async fn complete_item(
    repo: &State<Repository>,
    id: i64,
    show_done_items: bool,
    _logged_in: LoggedInUser,
) -> Result<Json<Items>> {
    let id = ItemId(id);
    repo.complete_item(id, Utc::now()).await?;
    let items = repo.get_items(None, None, show_done_items).await?;
    Ok(Json(items))
}

#[tracing::instrument(skip(repo, _logged_in), err)]
#[put("/items/<id>/undo?<show_done_items>")]
pub async fn undo_item(
    repo: &State<Repository>,
    id: i64,
    show_done_items: bool,
    _logged_in: LoggedInUser,
) -> Result<Json<Items>> {
    let id = ItemId(id);
    repo.undo_item(id).await?;
    let items = repo.get_items(None, None, show_done_items).await?;
    Ok(Json(items))
}

#[tracing::instrument(skip(repo, _logged_in), err)]
#[put("/items/<id>?<show_done_items>", format = "json", data = "<body>")]
pub async fn edit_item(
    repo: &State<Repository>,
    id: i64,
    body: Json<OpenItem>,
    show_done_items: bool,
    _logged_in: LoggedInUser,
) -> Result<Json<Items>> {
    let id = ItemId(id);
    repo.edit_item(id, body.into_inner()).await?;
    let items = repo.get_items(None, None, show_done_items).await?;
    Ok(Json(items))
}

#[tracing::instrument(skip(oauth_client, challenges))]
#[get("/auth")]
pub async fn auth(oauth_client: &State<OAuthClient>, challenges: &State<Challenges>) -> Redirect {
    let redirect_url = oauth_client.login(challenges.inner()).await;
    Redirect::to(String::from(redirect_url))
}

#[tracing::instrument(skip(code, state, oauth_client, challenges, login, jar))]
#[get("/auth/authorized?<code>&<state>")]
pub async fn login_authorized(
    code: String,
    state: String,
    oauth_client: &State<OAuthClient>,
    challenges: &State<Challenges>,
    login: &State<Login>,
    jar: &CookieJar<'_>,
) -> Result<Redirect, Unauthorized<()>> {
    match oauth_client
        .verify_challenge(
            AuthRequestQuery::new(AuthorizationCode::new(code), CsrfToken::new(state)),
            challenges,
        )
        .await
    {
        Ok(email) => {
            if login.is_allowed(&email) {
                info!("Storing login cookie");
                let mut cookie = Cookie::new(USER_COOKIE, 1.to_string());
                cookie.set_same_site(SameSite::Lax);
                cookie.set_secure(true);
                jar.add_private(cookie);
                Ok(Redirect::to("/"))
            } else {
                info!("Email {:?} is not invited", email);
                Err(Unauthorized(None))
            }
        }
        Err(e) => {
            warn!("Login failed: {:?}", e);
            Err(Unauthorized(None))
        }
    }
}
