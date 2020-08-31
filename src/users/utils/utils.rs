use super::hashify::create_hash;
use crate::database::pool::ConnectionPool;
use crate::errors::ServiceResult;
use crate::schema::users;
use crate::users::filters::blacklist::enforce::forbidden;
use crate::users::filters::profainity::enforce::beep;
use crate::users::filters::user_case_mapped::enforce::filter;
use crate::SETTINGS;
use unicode_normalization::UnicodeNormalization;

use crate::users::models;

#[derive(Debug)]
pub struct username_passowrd {
    pub normalised_username: String,
    pub hash: String,
}

pub async fn create_new_user(
    //    con: &ConnectionPool,
    username: &str,
    password: &str,
) -> ServiceResult<()> {
    let creds = create_new_user_runner(&username, &password)?;
    println!("{:?}", creds);
    Ok(())
}

fn create_new_user_runner(username: &str, password: &str) -> ServiceResult<username_passowrd> {
    let normalised_username = username.to_lowercase().nfc().collect::<String>();

    filter(&normalised_username)?;
    forbidden(&normalised_username)?;

    if SETTINGS.server.profainity_filter {
        beep(&normalised_username)?;
    }

    let hash = create_hash(password);
    Ok(username_passowrd {
        normalised_username,
        hash,
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn utils_create_new_user_runner() {
        let creds = create_new_user_runner("Realaravinth", "password").unwrap();
        assert_eq!(creds.normalised_username, "realaravinth");
    }

    #[test]
    fn utils_create_new_user_runner1() {
        let creds = create_new_user_runner("fuck", "password");
        match creds {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn utils_create_new_user_runner2() {
        let creds = create_new_user_runner(".htaccessasnc", "password");
        match creds {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }
}
