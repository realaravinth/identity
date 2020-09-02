use actix_session::Session;
use pow_sha256::PoW;

use crate::errors::{ServiceError, ServiceResult};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub async fn verify_pow(session: &Session, pow: &PoW<Vec<u8>>) -> ServiceResult<()> {
    let session_id = session.get::<String>("PoW").unwrap();
    if let Some(id) = session_id {
        let difficulty = u128::max_value() - u128::max_value() / 100_00000;

        if pow.is_sufficient_difficulty(difficulty) && pow.is_valid_proof(&id.as_bytes().to_vec()) {
            Ok(())
        } else {
            Err(ServiceError::PoWRequired)
        }
    } else {
        Err(ServiceError::PoWRequired)
    }
}

pub async fn gen_pow(session: &Session) -> ServiceResult<PoWConfig> {
    let session_id = session.get::<String>("PoW").unwrap();
    if let Some(_id) = session_id {
        Err(ServiceError::PoWRequired)
    } else {
        // TODO: Move difficulty into app state
        let difficulty = u128::max_value() - u128::max_value() / 100_00000;
        let phrase: String = thread_rng().sample_iter(&Alphanumeric).take(32).collect();
        session.set("PoW", &phrase).unwrap();
        Ok(PoWConfig { difficulty, phrase })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoWResponse {
    pub nonce: u64,
    pub result: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoWConfig {
    pub phrase: String,
    pub difficulty: u128,
}
