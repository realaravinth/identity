use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use super::utils::hashify::{create_hash, verify};
use super::utils::utils;

pub async fn sign_up(creds: web::Json<NewCreds>) -> impl Responder {
    HttpResponse::Ok()
        .set_header(actix_web::http::header::CONNECTION, "close")
        .finish()
}

pub async fn sign_in(creds: web::Json<Creds>, id: Identity) -> impl Responder {
    println!("success");
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

#[derive(Deserialize, Serialize)]
pub struct Creds {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewCreds {
    username: String,
    password: String,
    email: Option<String>,
}
