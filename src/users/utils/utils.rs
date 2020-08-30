use super::hashify::create_hash;
use crate::database::pool::ConnectionPool;
use crate::errors::ServiceResult;
use crate::schema::users;
use crate::settings::Settings;
use crate::users::filters::blacklist::enforce::forbidden;
use crate::users::filters::profainity::enforce::beep;
use crate::users::filters::user_case_mapped::enforce::filter;

use crate::users::models;
pub async fn create_new_user(
    con: &ConnectionPool,
    username: &str,
    password: &str,
) -> ServiceResult<()> {
    lazy_static! {
        pub static ref SETTINGS: Settings = Settings::new().unwrap();
    }
    let normalised_username = filter(username)?;

    if SETTINGS.server.profainity_filter {
        beep(username)?;
        beep(&normalised_username)?;
    }

    forbidden(username)?;
    forbidden(&normalised_username)?;

    let hash = create_hash(password);
    todo!(
        "
    username:
        existt()
        unicode normalise
        usercase mapped
        The big username blacklist
        Optional bad words filter
    passwords 
        hash()
    store()"
    );
    Ok(())
}
