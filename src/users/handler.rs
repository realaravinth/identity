use actix_identity::Identity;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};

use super::create_new_user;

use super::{Creds, NewCreds};
use crate::errors::ServiceResult;
use crate::pow::verify_pow;

pub async fn sign_up(
    session: Session,
    creds: web::Json<NewCreds>,
) -> ServiceResult<impl Responder> {
    let new_creds = creds.into_inner();
    let pow = &new_creds.pow;
    verify_pow(&session, &pow).await?;
    create_new_user(&new_creds.username, &new_creds.password).await?;

    Ok(HttpResponse::Ok()
        .set_header(actix_web::http::header::CONNECTION, "close")
        .finish())
}

pub async fn sign_in(
    session: Session,
    creds: web::Json<Creds>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    unimplemented!();
    Ok(HttpResponse::Ok().finish())
}

pub async fn sign_out(id: Identity) -> ServiceResult<impl Responder> {
    id.forget();
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body("You are successfully signed out"))
}
