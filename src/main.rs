extern crate env_logger;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate config;

#[macro_use]
extern crate lazy_static;

use actix_http::cookie::SameSite;
use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::{
    http,
    middleware::{Compress, DefaultHeaders, Logger},
    web, App, HttpResponse, HttpServer, Responder,
};

use listenfd::ListenFd;

use std::env;

mod settings;
mod users;

use settings::Settings;

lazy_static! {
    static ref SETTINGS: Settings = Settings::new().unwrap();
}

fn greet() -> impl Responder {
    HttpResponse::Ok().content_type("application/json").body(
        "
        { 'message' : 'received'}
        ",
    )
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use users::signin::{geti_id, index, sign_in};
    use users::signup::sign_up;
    let mut listenfd = ListenFd::from_env();
    let cookie_secret = &SETTINGS.server.cookie_secret;
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(cookie_secret.as_bytes())
                    .name("Authorization")
                    .max_age(20)
                    .domain("localhost")
                    .same_site(SameSite::Lax)
                    .secure(true),
            ))
            .wrap(Logger::default())
            .service(
                web::resource("/signin")
                    .route(web::post().to(sign_in))
                    .route(web::get().to(index))
                    .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
            )
            .service(
                web::resource("/signup")
                    .route(web::post().to(sign_up))
                    .route(web::get().to(geti_id))
                    .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
            )
    })
    .bind(format!("0.0.0.0:{}", &SETTINGS.server.port))
    .unwrap()
    .run()
    .await
}
