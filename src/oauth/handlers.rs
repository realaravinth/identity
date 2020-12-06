/*
* Copyright (C) 2020  Aravinth Manivannan <realaravinth@batsense.net>
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU Affero General Public License as
* published by the Free Software Foundation, either version 3 of the
* License, or (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU Affero General Public License for more details.
*
* You should have received a copy of the GNU Affero General Public License
* along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use actix::prelude::*;
use actix_web::{
    get, post,
    web::{self, ServiceConfig},
    HttpRequest,
};
use oxide_auth_actix::{
    Authorize, OAuthOperation, OAuthRequest, OAuthResource, OAuthResponse, Refresh, Resource,
    Token, WebError,
};

use deadpool_postgres::{Client, Pool};

use super::state::*;
use crate::errors::*;
use crate::Data;

static DENY_TEXT: &str = "<html>
This page should be accessed via an oauth token from the client in the example. Click
<a href=\"http://localhost:5000/api/oauth/authorize?response_type=code&client_id=LocalClient\">
here</a> to begin the authorization process.
</html>";

async fn get_email(username: &str, db_pool: &Pool) -> ServiceResult<String> {
    let statement = format!(
        " SELECT email_id FROM users WHERE username = '{}'",
        username
    );
    let client: Client = db_pool.get().await?;
    let command = client.prepare(&statement).await.unwrap();

    let mut rows = client.query(&command, &[]).await.unwrap();
    if let Some(val) = rows.pop() {
        if val.is_empty() {
            Err(ServiceError::UserDoesntExist)
        } else {
            Ok(val.get(0))
        }
    } else {
        Err(ServiceError::UserDoesntExist)
    }
}

#[get("/api/oauth/authorize")]
async fn get_authorize(
    req: OAuthRequest,
    state: web::Data<Data>,
) -> Result<OAuthResponse, WebError> {
    // GET requests should not mutate server state and are extremely
    // vulnerable accidental repetition as well as Cross-Site Request
    // Forgery (CSRF).
    state
        .oauth_state
        .send(Authorize(req).wrap(Extras::AuthGet))
        .await?
}

#[post("/api/oauth/authorize")]
async fn post_authorize(
    r: HttpRequest,
    req: OAuthRequest,
    state: web::Data<Data>,
) -> Result<OAuthResponse, WebError> {
    // Some authentication should be performed here in production cases
    state
        .oauth_state
        .send(Authorize(req).wrap(Extras::AuthPost(r.query_string().to_owned())))
        .await?
}

#[post("/api/oauth/token")]
async fn token(req: OAuthRequest, state: web::Data<Data>) -> Result<OAuthResponse, WebError> {
    state
        .oauth_state
        .send(Token(req).wrap(Extras::Nothing))
        .await?
}

#[post("/api/oauth/refresh")]
async fn refresh(
    req: OAuthRequest,
    state: web::Data<Addr<State>>,
) -> Result<OAuthResponse, WebError> {
    state.send(Refresh(req).wrap(Extras::Nothing)).await?
}

#[get("/api/oauth/resource")]
async fn resource(req: OAuthResource, state: web::Data<Data>) -> Result<OAuthResponse, WebError> {
    match state
        .oauth_state
        .send(Resource(req.into_request()).wrap(Extras::Nothing))
        .await?
    {
        Ok(_grant) => Ok(OAuthResponse::ok()
            .content_type("text/plain")?
            .body("Hello world!")),
        Err(Ok(e)) => Ok(e.body(DENY_TEXT)),
        Err(Err(e)) => Err(e),
    }
}

pub fn services(cfg: &mut ServiceConfig) {
    cfg.service(get_authorize);
    cfg.service(post_authorize);
    cfg.service(refresh);
    cfg.service(token);
    cfg.service(resource);
}
