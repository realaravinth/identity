use actix_web::web::{self, HttpResponse};

use super::{sign_in, sign_out, sign_up};
use crate::pow::send_pow_config;

#[cfg(not(tarpaulin_include))]
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api/signin")
            .route(web::post().to(sign_in))
            .route(web::get().to(send_pow_config))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
    cfg.service(
        web::resource("/api/signup")
            .route(web::post().to(sign_up))
            .route(web::get().to(send_pow_config))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
    cfg.service(
        web::resource("/api/signout")
            .route(web::post().to(sign_out))
            .route(web::get().to(|| HttpResponse::MethodNotAllowed()))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}
