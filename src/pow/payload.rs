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
use actix_session::Session;
use pow_sha256::PoW;
use redis_async::{resp::RespValue, resp_array};

use crate::errors::{ServiceError, ServiceResult};

use super::DIFFICULTY;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PoWConfig {
    pub phrase: String,
    pub difficulty: u32,
}

impl PoWConfig {
    pub fn new(phrase: &str, difficulty: u32) -> PoWConfig {
        // TODO: Move difficulty into app state(?)
        // ultimately, difficulty should be adjusted according to
        // server load
        PoWConfig {
            difficulty,
            phrase: phrase.into(),
        }
    }

    pub async fn verify_pow(
        session: &Session,
        pow: &PoW<Vec<u8>>,
        redis_addr: &Addr<RedisActor>,
    ) -> ServiceResult<()> {
        let session_id = session.get::<String>("PoW")?;
        if let Some(id) = session_id {
            let difficulty = get_difficulty(redis_addr, &id).await?;
            debug!(
                "PoW is sufficient difficulty: {}",
                pow.is_sufficient_difficulty(difficulty)
            );
            if pow.is_sufficient_difficulty(difficulty)
                && pow.is_valid_proof(&id.as_bytes().to_vec())
            {
                Ok(())
            } else {
                Err(ServiceError::PoWInvalid)
            }
        } else {
            Err(ServiceError::PoWRequired)
        }
    }
}

pub async fn get_difficulty(redis_addr: &Addr<RedisActor>, id: &str) -> ServiceResult<u128> {
    let difficulty = redis_addr
        .send(Command(resp_array!["GET", id]))
        .await
        .unwrap()
        .unwrap();
    if let RespValue::BulkString(val) = difficulty {
        let difficulty_factor: u128 = String::from_utf8(val).unwrap().parse().unwrap();
        let difficulty = u128::max_value() - u128::max_value() / difficulty_factor as u128;
        debug!("difficulty: {}", difficulty);
        Ok(difficulty)
    } else {
        Err(ServiceError::InternalServerError)
    }
}

#[cfg(tests)]
mod tests {
    use super::*;
    use actix_session::Session;

    #[actix_rt::test]
    async fn test_gen_pow() {
        let phrase = "testing";
        let pow_config = PoWConfig::new(phrase);
        assert_eq(PoWConfig.difficulty, DIFFICULTY);
        assert_eq(PoWConfig.phrase, phrase.into());
    }
}
