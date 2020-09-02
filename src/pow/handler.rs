use actix_session::Session;
use actix_web::{HttpResponse, Responder};

use super::utils::*;
use crate::errors::*;

pub async fn send_pow_config(session: Session) -> ServiceResult<impl Responder> {
    let config = gen_pow(&session).await?;
    Ok(HttpResponse::Ok().json(config))
}
