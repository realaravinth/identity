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
use actix_web::{
    middleware::{Compress, Logger},
    web, App, HttpResponse, HttpServer,
};
use argon2::{Config, ThreadMode, Variant, Version};

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
async fn main() -> std::io::Result<()> {
    let cookie_secret = &SETTINGS.server.cookie_secret;
    let password_config = Config {
        variant: Variant::Argon2i,
        version: Version::Version13,
        mem_cost: SETTINGS.password_difficulty.mem_cost,
        time_cost: SETTINGS.password_difficulty.time_cost,
        lanes: SETTINGS.password_difficulty.lanes,
        thread_mode: ThreadMode::Parallel,
        secret: &[],
        ad: &[],
        hash_length: 32,
    };

    let database_connection_pool = get_connection_pool(&SETTINGS.database.url);

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
            .configure(server::config)
            .wrap(Logger::default())
            .data(database_connection_pool.clone())
    })
    .bind(format!("0.0.0.0:{}", &SETTINGS.server.port))
    .unwrap()
    .run()
    .await
}
