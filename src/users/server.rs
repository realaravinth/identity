use actix_web::web::{self, HttpResponse};

use crate::users::handler::{init, sign_in, sign_out, sign_up, verify_pow};

#[cfg(not(tarpaulin_include))]
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api/signin")
            .route(web::post().to(sign_in))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
            .route(web::get().to(|| HttpResponse::MethodNotAllowed())),
    );
    cfg.service(
        web::resource("/api/signup")
            .route(web::post().to(sign_up))
            .route(web::get().to(|| HttpResponse::MethodNotAllowed()))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
    cfg.service(
        web::resource("/api/signout")
            .route(web::post().to(sign_out))
            .route(web::get().to(|| HttpResponse::MethodNotAllowed()))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
    cfg.service(
        web::resource("/api/init")
            .route(web::post().to(|| HttpResponse::MethodNotAllowed()))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
            .route(web::get().to(init)),
    );
    cfg.service(
        web::resource("/api/pow")
            .route(web::get().to(|| HttpResponse::MethodNotAllowed()))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
            .route(web::post().to(verify_pow)),
    );
}
