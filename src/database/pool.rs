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

use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;

use crate::SETTINGS;

#[cfg(not(tarpaulin_include))]
pub fn get_connection_pool() -> Pool {
    let config: crate::settings::Database = SETTINGS.database.clone();

    let mut pg_config: Config = config.into();
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    pg_config.manager = Some(mgr_config);
    pg_config.create_pool(NoTls).unwrap()
}
