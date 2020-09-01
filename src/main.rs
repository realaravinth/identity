extern crate argon2;
extern crate config;
extern crate regex;
extern crate unicode_normalization;
#[macro_use]
extern crate diesel;
extern crate env_logger;
extern crate num_cpus;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

use actix_http::cookie::SameSite;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_session::CookieSession;
use actix_web::{
    middleware::{Compress, Logger},
    App, HttpServer,
};

use regex::Regex;
use std::env;

mod database;
mod errors;
mod schema;
mod settings;
mod users;

use crate::users::filters::blacklist::tables::BLACKLIST;
use crate::users::filters::profainity::tables::PROFAINITY;
use crate::users::filters::user_case_mapped::tables::USERNAME_CASE_MAPPED;

use database::pool::get_connection_pool;
use settings::Settings;
use users::server;

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().unwrap();
    pub static ref RE_BLACKLIST: Regex = Regex::new(BLACKLIST).unwrap();
    pub static ref RE_PROFAINITY: Regex = Regex::new(PROFAINITY).unwrap();
    pub static ref RE_USERNAME_CASE_MAPPED: Regex = Regex::new(USERNAME_CASE_MAPPED).unwrap();
}

#[actix_rt::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> std::io::Result<()> {
    let cookie_secret = &SETTINGS.server.cookie_secret;

    let database_connection_pool = get_connection_pool(&SETTINGS.database.url);

    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .wrap(
                CookieSession::signed(&cookie_secret.as_bytes())
                    .domain(&SETTINGS.server.domain)
                    .name("shuttlecraft-session")
                    .path("/")
                    .secure(false),
            )
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(cookie_secret.as_bytes())
                    .name("Authorization")
                    .max_age(20)
                    .domain("localhost")
                    .same_site(SameSite::Lax)
                    .secure(true),
            ))
            .configure(server::config)
            .wrap(Logger::default())
            .data(database_connection_pool.clone())
    })
    .bind(format!("0.0.0.0:{}", &SETTINGS.server.port))
    .unwrap()
    .run()
    .await
}
