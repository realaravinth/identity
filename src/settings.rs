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

#[cfg(not(tarpaulin_include))]
impl From<Database> for deadpool_postgres::Config {
    fn from(config: Database) -> Self {
        let pool = Some(deadpool::managed::PoolConfig::new(config.pool as usize));
        let deadpool_postgres_config = deadpool_postgres::Config::new();
        deadpool_postgres::Config {
            user: Some(config.username),
            password: Some(config.password),
            port: Some(config.port as u16),
            dbname: Some(config.name),
            host: Some(config.hostname),
            pool,
            ..deadpool_postgres_config
        }
        .to_owned()
    }
}

impl Database {
    fn extract_database_url(url: &Url) -> Self {
        //        if url.scheme() != "postgres" || url.scheme() != "postgresql" {
        //            panic!("URL must be postgres://url, url found: {}", url.scheme());
        //        } else {

        debug!("Databse name: {}", url.path());
        let mut path = url.path().split("/");
        path.next();
        let name = path.next().expect("no database name").to_string();
        Database {
            port: url.port().expect("Enter database port").into(),
            hostname: url.host().expect("Enter database host").to_string(),
            username: url.username().into(),
            url: url.to_string(),
            password: url.password().expect("Enter database password").into(),
            name,
            pool: num_cpus::get() as u32,
        }
        //        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Redis {
    pub hostname: String,
    pub port: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub database: Database,
    pub server: Server,
    pub password_difficulty: PasswordDifficulty,
    pub redis: Redis,
}

#[cfg(not(tarpaulin_include))]
impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // setting default values
        s.set_default("password_difficulty.lanes", num_cpus::get().to_string())
            .expect("Couldn't get the number of CPUs");
        s.set_default("database.pool", num_cpus::get().to_string())
            .expect("Couldn't get the number of CPUs");

        // merging default config from file
        s.merge(File::with_name("config/default"))?;

        // setting RUN_MODE, default 0s development
        let env = env::var("RUN_MODE").unwrap_or("development".into());

        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        s.merge(File::with_name("config/local").required(false))?;

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
                let url = Url::parse(&val).expect("couldn't parse Database URL");
                let database_conf = Database::extract_database_url(&url);
                s = set_from_database_url(s, &database_conf);
            }
            Err(e) => println!("couldn't interpret DATABASE_URL: {}", e),
        }

        s = set_database_url(s);

        s.try_into()
    }
}

fn set_from_database_url(mut s: Config, database_conf: &Database) -> Config {
    s.set("database.username", database_conf.username.clone())
        .expect("Couldn't set database username");
    s.set("database.password", database_conf.password.clone())
        .expect("Couldn't access database password");
    s.set("database.hostname", database_conf.hostname.clone())
        .expect("Couldn't access database hostname");
    s.set("database.port", database_conf.port as i64)
        .expect("Couldn't access database port");
    s.set("database.name", database_conf.name.clone())
        .expect("Couldn't access database name");
    s
}

fn set_database_url(mut s: Config) -> Config {
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
    s
}

#[cfg(test)]
mod tests {

    use super::*;

    static PORT: u32 = 5432;
    static HOSTNAME: &str = "localhost";
    static USERNAME: &str = "postgres";
    static NAME: &str = "postgres";
    static POOL: u32 = 4;
    static PASSWORD: &str = "password";
    static URL: &str = "postgres://postgres:password@localhost:5432/postgres";

    fn get_database() -> Database {
        Database {
            port: PORT,
            hostname: HOSTNAME.into(),
            username: USERNAME.into(),
            name: NAME.into(),
            pool: POOL,
            password: PASSWORD.into(),
            url: URL.into(),
        }
    }

    #[test]
    fn extract_database_url_works() {
        let url = Url::parse(URL).expect("couldn't parse url");
        let generated_database = Database::extract_database_url(&url);

        assert_eq!(generated_database.username, USERNAME, "username checksout");
        assert_eq!(generated_database.port, PORT, "port checksout");
        assert_eq!(generated_database.name, NAME, "database name checksout");
        assert_eq!(generated_database.hostname, HOSTNAME, "hostname checksout");
        assert_eq!(generated_database.password, PASSWORD, "password checksout");
    }

    #[test]
    fn databse_deadpool_config() {
        let config: deadpool_postgres::Config = get_database().into();

        assert_eq!(config.user, Some(USERNAME.into()), "username checksout");
        assert_eq!(config.port, Some(PORT as u16), "port checksout");
        assert_eq!(config.dbname, Some(NAME.into()), "database name checksout");
        assert_eq!(config.host, Some(HOSTNAME.into()), "hostname checksout");
        assert_eq!(config.password, Some(PASSWORD.into()), "password checksout");
    }

    #[test]
    fn set_database_from_url_works() {
        let mut s = Config::new();
        let database = get_database();
        s = set_from_database_url(s, &database);

        assert_eq!(
            s.get::<String>("database.username")
                .expect("Couldn't access database username"),
            USERNAME,
            "username checksout"
        );
        assert_eq!(
            s.get::<String>("database.password")
                .expect("Couldn't access database password"),
            PASSWORD,
            "password checksout"
        );
        assert_eq!(
            s.get::<String>("database.hostname")
                .expect("Couldn't access database hostname"),
            HOSTNAME,
            "hostname checksout"
        );
        assert_eq!(
            s.get::<u32>("database.port")
                .expect("Couldn't access database port"),
            PORT,
            "port checksout"
        );
        assert_eq!(
            s.get::<String>("database.name")
                .expect("Couldn't access database name"),
            NAME,
            "database name checksout"
        );
    }

    #[test]
    fn set_database_url_works() {
        let mut s = Config::new();
        let database = get_database();
        s = set_from_database_url(s, &database);
        s = set_database_url(s);

        assert_eq!(
            s.get::<String>("database.url")
                .expect("Couldn't access url"),
            URL,
            "database url checksout"
        );
    }

    #[test]
    fn settings_from_file_works() {
        let config = Settings::new().unwrap();

        //        assert_eq!(config.database.username, "aravinth", "username checksout");
        //        assert_eq!(config.database.port, PORT, "port checksout");
        //        assert_eq!(config.database.name, "authdevel", "database name checksout");
        //        assert_eq!(config.database.hostname, HOSTNAME, "hostname checksout");
        //        assert_eq!(config.database.password, "", "password checksout");

        assert_eq!(config.database.username, USERNAME, "username checksout");
        assert_eq!(config.database.port, PORT, "port checksout");
        assert_eq!(config.database.name, NAME, "database name checksout");
        assert_eq!(config.database.hostname, HOSTNAME, "hostname checksout");
        assert_eq!(config.database.password, PASSWORD, "password checksout");
    }

    #[test]
    fn settings_from_env() {
        use std::env::{set_var, var};

        let original_url = var("DATABASE_URL");
        set_var("DATABASE_URL", URL);

        let config = Settings::new().unwrap();

        assert_eq!(config.database.username, USERNAME, "username checksout");
        assert_eq!(config.database.port, PORT, "port checksout");
        assert_eq!(config.database.name, NAME, "database name checksout");
        assert_eq!(config.database.hostname, HOSTNAME, "hostname checksout");
        assert_eq!(config.database.password, PASSWORD, "password checksout");

        if let Ok(url) = original_url {
            set_var("DATABASE_URL", url)
        }
    }
}
