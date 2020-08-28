use config::{Config, ConfigError, Environment, File};
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Password_Difficulty {
    pub mem_cost: u32,
    pub time_cost: u32,
    pub lanes: u32,
    pub hash_length: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub allow_registration: bool,
    pub port: u32,
    pub host: String,
    pub cookie_secret: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub port: u32,
    pub url: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub database: Database,
    pub server: Server,
    pub password_difficulty: Password_Difficulty,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name("config/default"))?;

        let env = env::var("RUN_MODE").unwrap_or("development".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        s.merge(File::with_name("config/local").required(false))?;

        s.merge(Environment::with_prefix("AUTH"))?;
        s.try_into()
    }
}
