use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;

pub type ConnectionPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_connection_pool() -> ConnectionPool {
    let connection_url = std::env::var("DATABASE_URL").expect("Database Url");
    let connection_manager = ConnectionManager::<PgConnection>::new(connection_url);
    let pool = r2d2::Pool::builder()
        .build(connection_manager)
        .expect("Connection Pool");

    pool
}
