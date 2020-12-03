#![warn(rust_2018_idioms, elided_lifetimes_in_paths)]

use pretty_env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use actix_rt;
use deadpool_postgres::{Client, Pool};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::error::SqlState;

//#[path = "../data.rs"]
//mod data;
#[path = "../database/mod.rs"]
mod database;
//#[path = "../pow/mod.rs"]
//mod pow;
#[path = "../settings.rs"]
mod settings;

#[path = "../errors.rs"]
mod errors;

//use data::Data;
use database::get_connection_pool;
use settings::Settings;

static USERNAME: &str = "a";
static PASSWORD: &str = "password";
static HASH: &str = "$argon2i$v=19$m=656,t=5,p=12$b0p3MnZIbDRwRzUzTDRhZW9weWpBWFc5ZkxUREN5eGE$57mYgK/vkOFFlbh1QMttQ1eBUrbkYdPawmkmQwevziw";
static EMAIL: &str = "batman@we.com";

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().expect("couldn't load settings");
}

#[actix_rt::main]
#[cfg(not(tarpaulin_include))]
async fn main() {
    let pool = get_connection_pool();
    let up_statement = include_str!("../../migrations/2020-08-04-165420_users/up.sql");
    let down_statement = include_str!("../../migrations/2020-08-04-165420_users/down.sql");
    pretty_env_logger::init();

    let client: Client = pool.get().await.unwrap();

    // crating tables
    info!("Creating tables");
    let up_command = client.prepare(&up_statement).await.unwrap();
    let down_command = client.prepare(&down_statement).await.unwrap();

    let table = client.execute(&up_command, &[]).await;

    if let Err(val) = table {
        if let Some(err) = val.code() {
            if err == &SqlState::DUPLICATE_TABLE {
                info!("table users exists");
                let _ = client.execute(&down_command, &[]).await.unwrap();
                let _ = client.execute(&up_command, &[]).await.unwrap();
            }
        }
    }

    // creating dummy user
    info!("Creating dummy users");

    let create_user_statement = format!(
        "INSERT INTO users(username, email_id, password) values('{}','{}','{}')",
        USERNAME, EMAIL, HASH
    );
    let create_user_command = client.prepare(&create_user_statement).await.unwrap();

    client.execute(&create_user_command, &[]).await;
}
