use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct NewCreds {
    username: String,
    password: String,
    email: Option<String>,
}

pub async fn sign_up(creds: web::Json<NewCreds>) -> impl Responder {
    println!("{:?}", creds);
    println!("success");
    HttpResponse::Ok().content_type("application/json").body(
        "
        { 'message' : 'received'}
        ",
    )
}
