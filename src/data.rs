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

use actix::prelude::*;
use actix_redis::{Command, RedisActor};
use deadpool_postgres::Pool;
use redis_async::{resp::RespValue, resp_array};

use crate::database::get_connection_pool;
use crate::pow::Counter;
use crate::SETTINGS;

#[cfg(not(tarpaulin_include))]
#[derive(Clone)]
pub struct Data {
    pub pool: Pool,
    pub counter_addr: Addr<Counter>,
    pub redis_addr: Addr<RedisActor>,
}

#[cfg(not(tarpaulin_include))]
impl Default for Data {
    fn default() -> Self {
        let redis_url = SETTINGS.redis.get_url();
        info!("Connecting to Redis at {}", &redis_url);
        let redis_addr = RedisActor::start(redis_url);

        Data {
            pool: get_connection_pool(),
            counter_addr: Counter::default().start(),
            redis_addr: redis_addr.to_owned(),
        }
    }
}
