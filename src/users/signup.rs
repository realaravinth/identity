use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::users::hashify::create_hash;

#[derive(Deserialize, Serialize, Debug)]
pub struct NewCreds {
    username: String,
    password: String,
    email: Option<String>,
}

pub async fn sign_up(creds: web::Json<NewCreds>) -> impl Responder {
    println!("{:?}", create_hash(&creds.password).await);
    HttpResponse::Ok()
        .set_header(actix_web::http::header::CONNECTION, "close")
        .finish()
}
