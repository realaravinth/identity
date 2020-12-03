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

//use data::Data;
use database::get_connection_pool;
use settings::Settings;

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
}
