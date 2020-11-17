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
use std::env;

use config::{Config, ConfigError, Environment, File};
use num_cpus;
use url::Url;

#[derive(Debug, Clone, Deserialize)]
pub struct PasswordDifficulty {
    pub mem_cost: u32,
    pub time_cost: u32,
    pub lanes: u32,
    pub hash_length: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    // TODO yet to be configured
    pub allow_registration: bool,
    pub port: u32,
    pub host: String,
    pub domain: String,
    pub cookie_secret: String,
    pub profanity_filter: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub port: u32,
    pub hostname: String,
    pub username: String,
    pub password: String,
    pub name: String,
    pub pool: u32,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub database: Database,
    pub server: Server,
    pub password_difficulty: PasswordDifficulty,
}

impl Settings {
    fn extract_database_url(url: &Url) -> Database {
        if url.scheme() != "postgres" || url.scheme() != "postgresql" {
            panic!("URL must be postgres://url");
        } else {
            Database {
                port: url.port().expect("Enter database port").into(),
                hostname: url.host().expect("Enter database host").to_string(),
                username: url.username().into(),
                url: url.to_string(),
                password: url.password().expect("Enter database password").into(),
                name: url.path().into(),
                pool: num_cpus::get() as u32,
            }
        }
    }

    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.set_default("password_difficulty.lanes", num_cpus::get().to_string())
            .expect("Couldn't get the number of CPUs");

        s.set_default("database.pool", num_cpus::get().to_string())
            .expect("Couldn't get the number of CPUs");

        s.merge(File::with_name("config/default"))?;

        let env = env::var("RUN_MODE").unwrap_or("development".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        s.merge(File::with_name("config/local").required(false))?;
        s.set(
            "database.url",
            format!(
                r"postgres://{}:{}@{}:{}/{}",
                s.get::<String>("database.username")
                    .expect("Couldn't access database username"),
                s.get::<String>("database.password")
                    .expect("Couldn't access database password"),
                s.get::<String>("database.hostname")
                    .expect("Couldn't access database hostname"),
                s.get::<String>("database.port")
                    .expect("Couldn't access database port"),
                s.get::<String>("database.name")
                    .expect("Couldn't access database name")
            ),
        )
        .expect("Couldn't set databse url");

        s.merge(Environment::with_prefix("AUTH"))?;

        match env::var("PORT") {
            Ok(val) => {
                s.set("server.port", val).unwrap();
                println!("")
            }
            Err(e) => println!("couldn't interpret PORT: {}", e),
        }

        match env::var("DATABASE_URL") {
            Ok(val) => {
                let database_conf = Settings::extract_database_url(
                    &Url::parse(&val).expect("couldn't parse Database URL"),
                );
                s.set("database.username", database_conf.username)
                    .expect("Couldn't set database username");
                s.set("database.password", database_conf.password)
                    .expect("Couldn't access database password");
                s.set("database.hostname", database_conf.hostname)
                    .expect("Couldn't access database hostname");
                s.set("database.port", database_conf.port as i64)
                    .expect("Couldn't access database port");
                s.set("database.name", database_conf.name)
                    .expect("Couldn't access database name");
            }
            Err(e) => println!("couldn't interpret DATABASE_URL: {}", e),
        }
        s.try_into()
    }
}
