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

use crate::SETTINGS;

use deadpool_postgres::{Config, Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;
//use diesel::prelude::*;
//use diesel::r2d2::{self, ConnectionManager};
//
//pub type ConnectionPool = r2d2::Pool<ConnectionManager<PgConnection>>;
//
pub fn get_connection_pool() -> Pool {
    //-> ConnectionPool {
    //    let connection_manager = ConnectionManager::<PgConnection>::new(connection_url);
    //    let pool = r2d2::Pool::builder()
    //        .build(connection_manager)
    //        .expect("Connection Pool");
    //
    //    pool
    let config: crate::settings::Database = SETTINGS.database.clone();

    let mut pg_config: Config = config.into();
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    pg_config.manager = Some(mgr_config);
    pg_config.create_pool(NoTls).unwrap()
}
