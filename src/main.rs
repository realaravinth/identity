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
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use actix_files::Files;
use actix_http::cookie::SameSite;
use actix_http::Error;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_session::CookieSession;
use actix_web::{
    dev,
    error::InternalError,
    http::StatusCode,
    middleware::{Compress, Logger},
    web, App, HttpServer,
};
use regex::Regex;

mod data;
mod database;
mod errors;
mod pow;
mod settings;
#[cfg(test)]
mod test;
mod users;

pub use data::Data;
use settings::Settings;
use users::BLACKLIST;
use users::PROFAINITY;
use users::USERNAME_CASE_MAPPED;

pub const POW_SESSION_DURATION: u64 = 60;

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
#[cfg(not(tarpaulin_include))]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    lazy_static::initialize(&SETTINGS);

    let data = Data::default();

    debug!("Configuration: {:#?}", *SETTINGS);

    HttpServer::new(move || {
        create_app()
            .wrap(Compress::default())
            .data(data.clone())
            .service(Files::new("/", "./frontend/dist").index_file("signin.html"))
            .wrap(Logger::default())
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

pub fn get_identity_service() -> IdentityService<CookieIdentityPolicy> {
    let cookie_secret = &SETTINGS.server.cookie_secret;
    IdentityService::new(
        CookieIdentityPolicy::new(cookie_secret.as_bytes())
            .name("Authorization")
            .max_age(20)
            .domain(&SETTINGS.server.domain)
            .secure(true),
    )
}

pub fn get_json_err() -> web::JsonConfig {
    web::JsonConfig::default().error_handler(|err, _| {
        debug!("JSON deserialization error: {:?}", &err);
        InternalError::new(err, StatusCode::BAD_REQUEST).into()
    })
}

pub fn get_cookie() -> CookieSession {
    let cookie_secret = &SETTINGS.server.cookie_secret;
    CookieSession::signed(&cookie_secret.as_bytes())
        .domain(&SETTINGS.server.domain)
        .name("pow")
        .same_site(SameSite::Strict)
        .max_age(POW_SESSION_DURATION as i64)
        .path("/")
        .secure(false) //TODO change dynamically between true and false
                       //                    based on mode=DEVEL
}

#[cfg(not(tarpaulin_include))]
pub fn create_app() -> App<
    impl actix_service::ServiceFactory<
        Config = (),
        Request = dev::ServiceRequest,
        Response = dev::ServiceResponse<actix_http::body::Body>,
        Error = Error,
        InitError = (),
    >,
    actix_http::body::Body,
> {
    App::new()
        .configure(pow::handlers::services)
        .configure(users::registration::handlers::services)
        .configure(users::authentication::handlers::services)
        .app_data(get_json_err())
        .wrap(get_cookie())
        .wrap(get_identity_service())
}
