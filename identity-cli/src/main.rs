#![warn(rust_2018_idioms, elided_lifetimes_in_paths)]

use pretty_env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use actix_rt;
use deadpool_postgres::Client;
use lazy_static::lazy_static;
use tokio_postgres::error::SqlState;

#[path = "../../src/database/mod.rs"]
mod database;
#[path = "../../src/settings.rs"]
mod settings;

//use data::Data;
use database::get_connection_pool;
use settings::Settings;

static USERNAME: &str = "a";
//static PASSWORD: &str = "password";
static HASH: &str = "$argon2i$v=19$m=656,t=5,p=12$b0p3MnZIbDRwRzUzTDRhZW9weWpBWFc5ZkxUREN5eGE$57mYgK/vkOFFlbh1QMttQ1eBUrbkYdPawmkmQwevziw";
static EMAIL: &str = "batman@we.com";

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().expect("couldn't load settings");
}

async fn create_table(client: &Client) -> std::result::Result<u64, tokio_postgres::Error> {
    let up_statement = include_str!("../../migrations/2020-08-04-165420_users/up.sql");

    let up_command = client.prepare(&up_statement).await.unwrap();

    client.execute(&up_command, &[]).await
}

async fn drop_table(client: &Client) {
    let down_statement = include_str!("../../migrations/2020-08-04-165420_users/down.sql");
    let down_command = client.prepare(&down_statement).await.unwrap();
    client.execute(&down_command, &[]).await.unwrap();
}

#[actix_rt::main]
#[cfg(not(tarpaulin_include))]
async fn main() {
    pretty_env_logger::init();

    let pool = get_connection_pool();
    let client: Client = pool.get().await.unwrap();

    // crating tables
    info!("Creating tables");

    if let Err(val) = create_table(&client).await {
        if let Some(err) = val.code() {
            if err == &SqlState::DUPLICATE_TABLE {
                info!("table users exists");
                drop_table(&client).await;
                create_table(&client).await.unwrap();
            }
        }
    }

    // creating dummy user
    info!("Creating dummy users");

    create_dummy_users(&client).await;
}

async fn create_dummy_users(client: &Client) {
    let create_user_statement = format!(
        "INSERT INTO users(username, email_id, password) values('{}','{}','{}')",
        USERNAME, EMAIL, HASH
    );
    let create_user_command = client.prepare(&create_user_statement).await.unwrap();

    client.execute(&create_user_command, &[]).await.unwrap();
}
