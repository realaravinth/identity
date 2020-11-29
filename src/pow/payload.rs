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

use actix_session::Session;
use pow_sha256::PoW;

use crate::errors::{ServiceError, ServiceResult};

use super::DIFFICULTY;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PoWConfig {
    pub phrase: String,
    pub difficulty: u128,
}

impl PoWConfig {
    pub fn new(phrase: &str) -> PoWConfig {
        // TODO: Move difficulty into app state(?)
        // ultimately, difficulty should be adjusted according to
        // server load
        PoWConfig {
            difficulty: DIFFICULTY,
            phrase: phrase.into(),
        }
    }

    pub fn verify_pow(session: &Session, pow: &PoW<Vec<u8>>) -> ServiceResult<()> {
        let session_id = session.get::<String>("PoW")?;
        if let Some(id) = session_id {
            if pow.is_sufficient_difficulty(DIFFICULTY)
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
