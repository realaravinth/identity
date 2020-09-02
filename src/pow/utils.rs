// Copyright (c) 2020 Aravinth T M <realaravinth@batsense.net>.
// See the COPYRIGHT file at the top-level directory of this
// distribution

//This program is free software; you can redistribute it and/or
//modify it under the terms of the GNU General Public License
//as published by the Free Software Foundation; either version 2
//of the License, or (at your option) any later version.

//This program is distributed in the hope that it will be useful,
//but WITHOUT ANY WARRANTY; without even the implied warranty of
//MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//GNU General Public License for more details.

//You should have received a copy of the GNU General Public License
//along with this program; if not, write to the Free Software
//Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.

use actix_session::Session;
use pow_sha256::PoW;

use crate::errors::{ServiceError, ServiceResult};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub async fn verify_pow(session: &Session, pow: &PoW<Vec<u8>>) -> ServiceResult<()> {
    let session_id = session.get::<String>("PoW")?;
    if let Some(id) = session_id {
        let difficulty = u128::max_value() - u128::max_value() / 100_00000;
        if pow.is_sufficient_difficulty(difficulty) && pow.is_valid_proof(&id.as_bytes().to_vec()) {
            Ok(())
        } else {
            Err(ServiceError::PoWRequired)
        }
    } else {
        Err(ServiceError::PoWRequired)
    }
}

pub async fn gen_pow(session: &Session) -> ServiceResult<PoWConfig> {
    let session_id = session.get::<String>("PoW");
    if let Some(_id) = session_id? {
        Err(ServiceError::PoWRequired)
    } else {
        // TODO: Move difficulty into app state
        let difficulty = u128::max_value() - u128::max_value() / 100_00000;
        let phrase: String = thread_rng().sample_iter(&Alphanumeric).take(32).collect();
        session.set("PoW", &phrase)?;
        Ok(PoWConfig { difficulty, phrase })
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub struct PoWResponse {
    pub nonce: u64,
    pub result: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PoWConfig {
    pub phrase: String,
    pub difficulty: u128,
}

#[cfg(tests)]
mod tests {
    use super::*;
    use actix_session::Session;

    #[actix_rt::test]
    async fn test_gen_pow() {
        let session = Session::new();
        println!("{:?}", session);
    }
}
