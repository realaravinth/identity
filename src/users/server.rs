use actix_web::web::{self, HttpResponse};

use crate::users::handler::{sign_in, sign_out, sign_up};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/signin")
            .route(web::post().to(sign_in))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
            .route(web::get().to(|| HttpResponse::MethodNotAllowed())),
    );
    cfg.service(
        web::resource("/signup")
            .route(web::post().to(sign_up))
            .route(web::get().to(|| HttpResponse::MethodNotAllowed()))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
    cfg.service(
        web::resource("/signout")
            .route(web::post().to(sign_out))
            .route(web::get().to(|| HttpResponse::MethodNotAllowed()))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}
