use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type ConnectionPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_connection_pool(connection_url: &String) -> ConnectionPool {
    let connection_manager = ConnectionManager::<PgConnection>::new(connection_url);
    let pool = r2d2::Pool::builder()
        .build(connection_manager)
        .expect("Connection Pool");

    pool
}
