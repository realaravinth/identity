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
use argon2::{self, Config, ThreadMode, Variant, Version};
use pow_sha256::PoW;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use unicode_normalization::UnicodeNormalization;
use validator::Validate;
use validator_derive::Validate;

use super::{beep, filter, forbidden};
use crate::errors::*;
use crate::SETTINGS;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UnvalidatedRegisterCreds {
    pub username: String,
    pub email_id: Option<String>,
    pub password: String,
    pub pow: PoW<Vec<u8>>,
}

#[derive(Debug, Default, Clone, PartialEq, Validate, Deserialize, Serialize)]
pub struct RegisterCreds {
    pub username: String,
    #[validate(email)]
    pub email_id: Option<String>,
    pub password: String,
}
//impl AsRef<RegisterCreds> for RegisterCreds {
//    fn as_ref(&self) -> &RegisterCreds {
//        &self
//    }
//}
//
//impl AsMut<RegisterCreds> for RegisterCreds {
//    fn as_mut<'a>(&'a mut self) -> &'a mut Self {
//        &mut self
//    }
//}
impl UnvalidatedRegisterCreds {
    pub fn process(&self) -> ServiceResult<RegisterCreds> {
        let creds = RegisterCreds::new()
            .set_email(&self.email_id)?
            .set_username(&self.username)
            .validate_fields()?
            .set_password(&self.password)?
            .build();
        Ok(creds)
    }
}

impl RegisterCreds {
    //    pub fn create_register_creds<'a>(&'a mut self) -> ServiceResult<Self> {
    //        let creds = RegisterCreds::new()
    //            .set_email(&u_creds.email_id)?
    //            .set_username(&u_creds.username)
    //            .validate_fields()?
    //            .set_password(&u_creds.password)?
    //            .build();
    //        Ok(creds)
    //    }
    fn new() -> Self {
        let registered_creds: RegisterCreds = Default::default();
        registered_creds
    }

    fn set_username<'a>(&'a mut self, username: &str) -> &'a mut Self {
        self.username = clean(username)
            .to_lowercase()
            .nfc()
            .collect::<String>()
            .trim()
            .to_owned();
        self
    }

    fn set_email<'a>(&'a mut self, email_id: &Option<String>) -> ServiceResult<&'a mut Self> {
        if let Some(email) = email_id {
            self.email_id = Some(email.trim().to_owned());
            self.validate()?;
        }
        Ok(self)
    }

    fn validate_fields<'a>(&'a mut self) -> ServiceResult<&'a mut Self> {
        filter(&self.username)?;
        forbidden(&self.username)?;
        beep(&self.username)?;
        Ok(self)
    }

    fn set_password<'a>(&'a mut self, password: &str) -> ServiceResult<&'a mut Self> {
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

        let salt: String = thread_rng().sample_iter(&Alphanumeric).take(32).collect();
        self.password = argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config)?;
        Ok(self)
    }

    fn build(&mut self) -> Self {
        self.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utils_register_builer() {
        let registered_creds = RegisterCreds::new()
            .set_password("password")
            .unwrap()
            .set_username("realaravinth")
            .set_email(&Some("batman@we.net".into()))
            .unwrap()
            .validate_fields()
            .unwrap()
            .build();

        assert_eq!(registered_creds.username, "realaravinth");
        assert_eq!(registered_creds.email_id, Some("batman@we.net".into()));
    }

    #[test]
    fn utils_register_email_err() {
        let mut email_err = RegisterCreds::new()
            .set_password("password")
            .unwrap()
            .set_username("realaravinth")
            .build();
        assert_eq!(
            email_err.set_email(&Some("sdfasdf".into())),
            Err(ServiceError::NotAnEmail)
        );
    }

    #[test]
    fn utils_create_new_organisation() {
        let password = "somepassword";
        let org = RegisterCreds::new()
            .set_email(&Some("batman@we.net".into()))
            .unwrap()
            .set_username("Realaravinth")
            .validate_fields()
            .unwrap()
            .set_password(password)
            .unwrap()
            .build();

        assert_eq!(org.username, "realaravinth");

        assert!(
            argon2::verify_encoded(&org.password, password.as_bytes()).unwrap(),
            "verify hahsing"
        );
    }

    #[test]
    fn utils_create_new_profane_organisation() {
        let mut profane_org = RegisterCreds::new();
        profane_org.set_username("fuck");

        assert_eq!(
            profane_org.validate_fields(),
            Err(ServiceError::UsernameError)
        );
    }

    #[test]
    fn utils_create_new_forbidden_organisation() {
        let mut forbidden_org = RegisterCreds::new().set_username("htaccessasnc").build();

        assert_eq!(
            forbidden_org.validate_fields(),
            Err(ServiceError::UsernameError)
        );
    }
}
