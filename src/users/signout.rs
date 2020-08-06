use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::{web, HttpResponse, Responder};

pub async fn geti_id(id: Identity) -> impl Responder {
    id.forget();
    HttpResponse::Ok()
        .content_type("text/html")
        .body("You are successfully signed out")
}
