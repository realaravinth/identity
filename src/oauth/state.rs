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
use oxide_auth::{
    endpoint::{Endpoint, OwnerConsent, OwnerSolicitor, Solicitation},
    frontends::simple::endpoint::{ErrorInto, FnSolicitor, Generic, Vacant},
    primitives::prelude::{AuthMap, Client, ClientMap, RandomGenerator, Scope, TokenMap},
};
use oxide_auth_actix::{
    Authorize, OAuthMessage, OAuthOperation, OAuthRequest, OAuthResource, OAuthResponse, Refresh,
    Resource, Token, WebError,
};

use url::Url;

pub struct State {
    endpoint: Generic<
        ClientMap,
        AuthMap<RandomGenerator>,
        TokenMap<RandomGenerator>,
        Vacant,
        Vec<Scope>,
        fn() -> OAuthResponse,
    >,
}

pub enum Extras {
    AuthGet,
    AuthPost(String),
    Nothing,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RegisterClient {
    pub name: String,
    pub redirect_uri: Url,
}

impl State {
    pub fn preconfigured() -> Self {
        State {
            endpoint: Generic {
                // A registrar with one pre-registered client
                registrar: vec![Client::public(
                    "LocalClient",
                    "http://localhost:8021/endpoint"
                        .parse::<url::Url>()
                        .unwrap()
                        .into(),
                    "default-scope".parse().unwrap(),
                )]
                .into_iter()
                .collect(),
                //registrar: ClientMap::new(),
                // Authorization tokens are 16 byte random keys to a memory hash map.
                authorizer: AuthMap::new(RandomGenerator::new(16)),
                // Bearer tokens are also random generated but 256-bit tokens, since they live longer
                // and this example is somewhat paranoid.
                //
                // We could also use a `TokenSigner::ephemeral` here to create signed tokens which can
                // be read and parsed by anyone, but not maliciously created. However, they can not be
                // revoked and thus don't offer even longer lived refresh tokens.
                issuer: TokenMap::new(RandomGenerator::new(16)),

                solicitor: Vacant,

                // A single scope that will guard resources for this endpoint
                scopes: vec!["default-scope".parse().unwrap()],

                response: OAuthResponse::ok,
            },
        }
    }

    pub fn with_solicitor<'a, S>(
        &'a mut self,
        solicitor: S,
    ) -> impl Endpoint<OAuthRequest, Error = WebError> + 'a
    where
        S: OwnerSolicitor<OAuthRequest> + 'static,
    {
        ErrorInto::new(Generic {
            authorizer: &mut self.endpoint.authorizer,
            registrar: &mut self.endpoint.registrar,
            issuer: &mut self.endpoint.issuer,
            solicitor,
            scopes: &mut self.endpoint.scopes,
            response: OAuthResponse::ok,
        })
    }
}

impl Actor for State {
    type Context = Context<Self>;
}

//impl Handler<RegisterClient> for State {
//    type Result = ();
//    fn handle(&mut self, msg: RegisterClient, _ctx: &mut Self::Context) -> Self::Result {
//        let client = Client::public(
//            &msg.name,
//            msg.redirect_uri.into(),
//            "default-scope".parse().unwrap(),
//        );
//        self.endpoint.registrar.register_client(client);
//    }
//}

impl<Op> Handler<OAuthMessage<Op, Extras>> for State
where
    Op: OAuthOperation,
{
    type Result = Result<Op::Item, Op::Error>;

    fn handle(&mut self, msg: OAuthMessage<Op, Extras>, _: &mut Self::Context) -> Self::Result {
        let (op, ex) = msg.into_inner();

        match ex {
            Extras::AuthGet => {
                let solicitor =
                    FnSolicitor(move |_: &mut OAuthRequest, pre_grant: Solicitation| {
                        // This will display a page to the user asking for his permission to proceed. The submitted form
                        // will then trigger the other authorization handler which actually completes the flow.
                        OwnerConsent::InProgress(
                            OAuthResponse::ok()
                                .content_type("text/html")
                                .unwrap()
                                .body(&consent_page_html("/api/oauth/authorize".into(), pre_grant)),
                        )
                    });

                op.run(self.with_solicitor(solicitor))
            }
            Extras::AuthPost(query_string) => {
                let solicitor = FnSolicitor(move |_: &mut OAuthRequest, _: Solicitation| {
                    if query_string.contains("allow") {
                        OwnerConsent::Authorized("dummy user".to_owned())
                    } else {
                        OwnerConsent::Denied
                    }
                });

                op.run(self.with_solicitor(solicitor))
            }
            _ => op.run(&mut self.endpoint),
        }
    }
}

fn consent_page_html(route: &str, solicitation: Solicitation) -> String {
    macro_rules! template {
        () => {
"<html>'{0:}' (at {1:}) is requesting permission for '{2:}'
<form method=\"post\">
    <input type=\"submit\" value=\"Accept\" formaction=\"{5:}?response_type=code&client_id={3:}{4:}&allow=true\">
    <input type=\"submit\" value=\"Deny\" formaction=\"{5:}?response_type=code&client_id={3:}{4:}&deny=true\">
</form>
</html>"
        };
    }

    let grant = solicitation.pre_grant();
    let state = solicitation.state();

    format!(
        template!(),
        grant.client_id,
        grant.redirect_uri,
        grant.scope,
        grant.client_id,
        if let Some(state) = state {
            format!("&state={}", state)
        } else {
            String::new()
        },
        &route
    )
}
