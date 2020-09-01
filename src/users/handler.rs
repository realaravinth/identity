use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_session::{CookieSession, Session};
use actix_web::{web, HttpResponse, Responder};
use pow_sha256::PoW;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::marker::PhantomData;

use super::models::*;
use super::utils::utils::create_new_user;
use crate::errors::{ServiceError, ServiceResult};

pub async fn sign_up(session: Session, creds: web::Json<NewCreds>) -> impl Responder {
    set_cookie(&session).unwrap();
    create_new_user(&creds.username, &creds.password)
        .await
        .unwrap();

    HttpResponse::Ok()
        .set_header(actix_web::http::header::CONNECTION, "close")
        .finish()
}

pub async fn init(session: Session) -> impl Responder {
    //set_cookie(&session).unwrap();

    let session_id = session.get::<String>("PoW").unwrap();

    if let Some(_id) = session_id {
        HttpResponse::BadRequest().finish()
    } else {
        let difficulty = u128::max_value() - u128::max_value() / 100_00000;
        let phrase: String = thread_rng().sample_iter(&Alphanumeric).take(32).collect();
        session.set("PoW", &phrase).unwrap();
        let config = PoWConfig { difficulty, phrase };
        HttpResponse::Ok().json(config)
    }
}

pub async fn verify_pow(session: Session, pow: web::Json<PoW<Vec<u8>>>) -> impl Responder {
    let session_id = session.get::<String>("PoW").unwrap();
    if let Some(id) = session_id {
        let y = pow.into_inner();
        if y.is_valid_proof(&id.as_bytes().to_vec()) {
            HttpResponse::Ok().content_type("text/html").body("ALright")
        } else {
            HttpResponse::BadRequest().finish()
        }
    } else {
        HttpResponse::BadRequest().finish()
    }
}
pub async fn sign_in(session: Session, creds: web::Json<Creds>, id: Identity) -> impl Responder {
    set_cookie(&session).unwrap();

    let x = id.identity();
    if x.is_none() {
        id.remember("Batman".to_owned());
    } else {
        println!("{} {} Id: {}", creds.username, creds.password, x.unwrap());
    }
    id.remember("Postman".to_owned());
    HttpResponse::Ok().finish()
}

pub async fn sign_out(id: Identity) -> impl Responder {
    id.forget();
    HttpResponse::Ok()
        .content_type("text/html")
        .body("You are successfully signed out")
}

fn set_cookie(session: &Session) -> ServiceResult<()> {
    let session_id = session.get::<String>("session-id").unwrap();

    if let Some(_id) = session_id {
        ()
    } else {
        let new_id: String = thread_rng().sample_iter(&Alphanumeric).take(32).collect();
        session.set("sessionid", new_id).unwrap()
    }

    Ok(())
}
