use argon2::{self, Config, ThreadMode, Variant, Version};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn generate_salt() -> String {
    let salt: String = thread_rng().sample_iter(&Alphanumeric).take(32).collect();
    salt
}

#[derive(Debug)]
pub struct Hashed {
    hash: String,
    salt: String,
}

pub async fn create_hash(password: &str) -> Hashed {
    let salt = generate_salt();
    let config = Config {
        variant: Variant::Argon2i,
        version: Version::Version13,
        mem_cost: 65536,
        time_cost: 5,
        lanes: 12,
        thread_mode: ThreadMode::Parallel,
        secret: &[],
        ad: &[],
        hash_length: 32,
    };
    let hash = argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap();
    Hashed { hash, salt }
}
