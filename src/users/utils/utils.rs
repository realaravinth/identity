use super::hashify::create_hash;
use crate::database::pool::ConnectionPool;
use crate::errors::ServiceResult;
use crate::schema::users;
use crate::settings::Settings;
use crate::users::filters::blacklist::enforce::forbidden;
use crate::users::filters::profainity::enforce::beep;
use crate::users::filters::user_case_mapped::enforce::filter;
use unicode_normalization::UnicodeNormalization;

use crate::users::models;

pub struct username_passowrd {
    pub normalised_username: String,
    pub hash: String,
}

pub async fn create_new_user_(
    con: &ConnectionPool,
    username: &str,
    password: &str,
) -> ServiceResult<()> {
    let creds = create_new_user_runner(&username, &password).await?;
    Ok(())
}

pub async fn create_new_user_runner(
    //    con: &ConnectionPool,
    username: &str,
    password: &str,
) -> ServiceResult<username_passowrd> {
    lazy_static! {
        pub static ref SETTINGS: Settings = Settings::new().unwrap();
    }
    let normalised_username = username.to_lowercase().nfc().collect::<String>();

    filter(&normalised_username)?;
    forbidden(&normalised_username)?;

    if SETTINGS.server.profainity_filter {
        beep(&normalised_username)?;
    }

    let hash = create_hash(password).await;
    Ok(username_passowrd {
        normalised_username,
        hash,
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[actix_rt::test]
    async fn utils_create_new_user_runner() {
        let creds = create_new_user_runner("Realaravinth", "password")
            .await
            .unwrap();
        assert_eq!(creds.normalised_username, "realaravinth");
    }

    #[actix_rt::test]
    async fn utils_create_new_user_runner1() {
        let creds = create_new_user_runner("fuck", "password").await;
        match creds {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }

    #[actix_rt::test]
    async fn utils_create_new_user_runner2() {
        let creds = create_new_user_runner(".htaccessasnc", "password").await;
        match creds {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }
}
