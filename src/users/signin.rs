use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Creds {
    username: String,
    password: String,
}

pub async fn geti_id(id: Identity) -> impl Responder {
    let x = id.identity();
    if x.is_none() {
        id.remember("Batman".to_owned());
    }
    HttpResponse::Ok()
        .content_type("text/html")
        .body("<a href='/signin'> hellow</a>")
}
pub async fn index(id: Identity) -> impl Responder {
    let link = "<a href='/signup'>get cookie</a>";
    let content = if let Some(id) = id.identity() {
        format!("Welcome! {} {}", id, link)
    } else {
        format!("{}", link)
    };
    HttpResponse::Ok().content_type("text/html").body(content)
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
