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




use unicode_normalization::UnicodeNormalization;

use super::create_hash;
use crate::errors::ServiceResult;
use crate::users::filters::blacklist::enforce::forbidden;
use crate::users::filters::profainity::enforce::beep;
use crate::users::filters::user_case_mapped::enforce::filter;
use crate::users::InsertableCreds;
use crate::SETTINGS;

pub async fn create_new_user(
    //    con: &ConnectionPool,
    username: &str,
    password: &str,
) -> ServiceResult<()> {
    let creds = create_new_user_runner(&username, &password)?;
    println!("{:?}", creds);
    Ok(())
}

fn create_new_user_runner(username: &str, password: &str) -> ServiceResult<InsertableCreds> {
    let normalised_username = username.to_lowercase().nfc().collect::<String>();
    // TODO chain ffilters
    filter(&normalised_username)?;
    forbidden(&normalised_username)?;

    if SETTINGS.server.profainity_filter {
        beep(&normalised_username)?;
    }

    let hash = create_hash(password)?;
    Ok(InsertableCreds::default(normalised_username, hash))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::errors::*;
    #[test]
    fn utils_create_new_user_runner() {
        let creds = create_new_user_runner("Realaravinth", "password").unwrap();
        let profanity = create_new_user_runner("fuck", "password");
        let forbidden_creds = create_new_user_runner(".htaccessasnc", "password");

        assert_eq!(creds.username, "realaravinth");
        assert_eq!(profanity, Err(ServiceError::CharError));
        assert_eq!(forbidden_creds, Err(ServiceError::CharError));
    }
}
