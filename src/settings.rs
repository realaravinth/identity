/*
* Wagon is an independent mailing list manager
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



use config::{Config, ConfigError, Environment, File};
use num_cpus;
use std::env;

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
    pub profainity_filter: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    port: u32,
    hostname: String,
    username: String,
    pub url: String,
    password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub database: Database,
    pub server: Server,
    pub password_difficulty: PasswordDifficulty,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

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
                s.get::<String>("database.database_name")
                    .expect("Couldn't access database name")
            ),
        )
        .expect("Couldn't set databse url");
        s.set("password_difficulty.lanes", num_cpus::get().to_string())
            .expect("Couldn't get the number of CPUs");
        s.merge(Environment::with_prefix("AUTH"))?;
        s.try_into()
    }
}
