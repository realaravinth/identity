use crate::errors::ServiceError;
use crate::settings::Settings;
use crate::SETTINGS;
use argon2::{self, verify_encoded, Config, ThreadMode, Variant, Version};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn generate_salt() -> String {
    let salt: String = thread_rng().sample_iter(&Alphanumeric).take(32).collect();
    salt
}

pub fn create_hash(password: &str) -> String {
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
    let hash = argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap();
    hash
}

pub fn verify(hash: &str, password: &str) -> Result<(), ServiceError> {
    if verify_encoded(&hash, password.as_bytes()).unwrap() {
        Ok(())
    } else {
        Err(ServiceError::AuthorizationRequired)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_crate_hash() {
        let password = "somepassword";
        let hash = create_hash(&password);
        assert!(argon2::verify_encoded(&hash, password.as_bytes()).unwrap());
    }
    fn test_unauthorized_verify() {
        let password = "somepassword";
        let hash = create_hash(&password);

        assert!(
            verify(&hash, "asdasd").is_err(),
            ServiceError::AuthorizationRequired
        );
    }

    #[actix_rt::test]
    async fn test_sucess_verify() {
        let password = "somepassword";
        let hash = create_hash(&password);

        assert!(verify(&hash, &password).is_ok(), ());
    }
}
