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

use super::{PoWConfig, DIFFICULTY};
use crate::errors::*;

use actix_session::Session;
use pow_sha256::PoW;

pub async fn verify_pow(session: &Session, pow: &PoW<Vec<u8>>) -> ServiceResult<()> {
    let session_id = session.get::<String>("PoW")?;
    if let Some(id) = session_id {
        if pow.is_sufficient_difficulty(DIFFICULTY) && pow.is_valid_proof(&id.as_bytes().to_vec()) {
            Ok(())
        } else {
            Err(ServiceError::PoWRequired)
        }
    } else {
        Err(ServiceError::PoWRequired)
    }
}
