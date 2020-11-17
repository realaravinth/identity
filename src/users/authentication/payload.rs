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

use ammonia::clean;
use argon2::verify_encoded;
use derive_more::AsRef;
use pow_sha256::PoW;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use unicode_normalization::UnicodeNormalization;
use validator::Validate;
use validator_derive::Validate;

use super::{beep, filter, forbidden, verify};
use crate::errors::*;
use crate::SETTINGS;

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct LoginCreds {
    pub username: String,
    pub password: String,
}

//impl LoginCreds {
//    fn verify(&self) -> ServiceResult<()> {
//        if verify_encoded(&hash, password.as_bytes())? {
//            Ok(())
//        } else {
//            Err(ServiceError::AuthorizationRequired)
//        }
//    }
//}
