/*
* Wagon is an independent mailing list manager
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




use crate::errors::*;
use crate::SETTINGS;
use argon2::{self, verify_encoded, Config, ThreadMode, Variant, Version};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn generate_salt() -> String {
    let salt: String = thread_rng().sample_iter(&Alphanumeric).take(32).collect();
    salt
}

pub fn create_hash(password: &str) -> ServiceResult<String> {
    let config = Config {
        variant: Variant::Argon2i,
        version: Version::Version13,
        mem_cost: SETTINGS.password_difficulty.mem_cost,
        time_cost: SETTINGS.password_difficulty.time_cost,
        lanes: SETTINGS.password_difficulty.lanes,
        thread_mode: ThreadMode::Parallel,
        secret: &[],
        ad: &[],
        hash_length: 32,
    };
    let salt = generate_salt();
    let hash = argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config)?;
    Ok(hash)
}

pub fn verify(hash: &str, password: &str) -> ServiceResult<()> {
    if verify_encoded(&hash, password.as_bytes())? {
        Ok(())
    } else {
        Err(ServiceError::AuthorizationRequired)
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_crate_hash() {
        let password = "somepassword";
        let hash = create_hash(&password).unwrap();
        assert!(argon2::verify_encoded(&hash, password.as_bytes()).unwrap());
    }

    #[test]
    fn test_verify() {
        let password = "somepassword";
        let hash = create_hash(&password).unwrap();

        assert_eq!(
            verify(&hash, "asdasd"),
            Err(ServiceError::AuthorizationRequired)
        );

        assert!(verify(&hash, &password).is_ok(), ());
    }
}
