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

#![warn(rust_2018_idioms, elided_lifetimes_in_paths)]
use pretty_env_logger;
#[macro_use]
extern crate log;
extern crate actix;
extern crate argon2;
extern crate config;
extern crate futures;
extern crate regex;
extern crate unicode_normalization;
extern crate uuid;
#[macro_use]
extern crate num_cpus;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate validator;
#[macro_use]
extern crate validator_derive;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_session::CookieSession;
use actix_web::{
    error::InternalError,
    http::StatusCode,
    middleware::{Compress, Logger},
    web, App, HttpServer,
};

use regex::Regex;
use std::env;

mod database;
mod errors;
mod pow;
mod routes;
mod settings;
mod users;

use crate::users::BLACKLIST;
use crate::users::PROFAINITY;
use crate::users::USERNAME_CASE_MAPPED;

use database::get_connection_pool;
use routes::routes;
use settings::Settings;

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().expect("couldn't load settings");
    pub static ref RE_BLACKLIST: Regex =
        Regex::new(BLACKLIST).expect("couldn't setup blacklist list filter");
    pub static ref RE_PROFAINITY: Regex =
        Regex::new(PROFAINITY).expect("coudln't setup profainity filter");
    pub static ref RE_USERNAME_CASE_MAPPED: Regex =
        Regex::new(USERNAME_CASE_MAPPED).expect("coudln't setup username case mapped filter");
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let cookie_secret = &SETTINGS.server.cookie_secret;

    let database_connection_pool = get_connection_pool();

    pretty_env_logger::init();
    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .app_data(web::JsonConfig::default().error_handler(|err, _| {
                debug!("JSON deserialization error: {:?}", &err);
                InternalError::new(err, StatusCode::BAD_REQUEST).into()
            }))
            .wrap(
                CookieSession::signed(&cookie_secret.as_bytes())
                    .domain(&SETTINGS.server.domain)
                    .name("shuttlecraft-session")
                    .path("/")
                    .secure(false),
            )
            .wrap(
                CookieSession::signed(&cookie_secret.as_bytes())
                    .domain(&SETTINGS.server.domain)
                    .name("on")
                    .path("/")
                    .secure(false),
            )
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(cookie_secret.as_bytes())
                    .name("Authorization")
                    .max_age(20)
                    .domain(&SETTINGS.server.domain)
                    .secure(true),
            ))
            .configure(routes)
            .wrap(Logger::default())
        //           .data(database_connection_pool.clone())
    })
    .bind(format!(
        "{}:{}",
        &SETTINGS.server.host, &SETTINGS.server.port
    ))
    .expect(&format!(
        "Couldn't bind to IP address: {} and port: {}, are they avaiable?",
        &SETTINGS.server.host, &SETTINGS.server.port
    ))
    .run()
    .await
}
