use actix_cors::Cors;
use actix_http::cookie::SameSite;
use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::{
    http,
    middleware::{Compress, Logger},
    web, App, HttpResponse, HttpServer, Responder,
};
use env_logger::Env;
use listenfd::ListenFd;
use serde::{Deserialize, Serialize};
use std::env;

mod users;

fn greet() -> impl Responder {
    HttpResponse::Ok().content_type("application/json").body(
        "
        { 'message' : 'received'}
        ",
    )
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use users::signin::{geti_id, index, sign_in};
    use users::signup::sign_up;
    //
    // DONT
    //      FOGET TO CHANGE COOKIE_SECRET
    //
    //
    //
    //
    //
    let mut listenfd = ListenFd::from_env();
    //let metrics = Metrics::new("/metrics", "actix_web_mw_test");
    env::set_var("RUST_LOG", "actix_web=info");
    env::set_var(
        "cookie_secret",
        "Zae0OOxf^bOJ#zN^&k7VozgW&QAx%n02TQFXpRMG4cCU0xMzgu3dna@tQ9dvc&TlE6p*n#kXUdLZJCQsuODIV%r$@o4%770ePQB7m#dpV!optk01NpY0@615w5e2Br4d"
    );
    env_logger::init();
    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(Compress::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(env::var("cookie_secret").unwrap().as_bytes())
                    .name("Authorization")
                    .max_age(20)
                    .domain("localhost")
                    .same_site(SameSite::Lax)
                    .secure(true),
            ))
            .wrap(
                Cors::new() // <- Construct CORS middleware builder
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .wrap(Logger::default())
            .service(
                web::resource("/signin")
                    .route(web::post().to(sign_in))
                    .route(web::get().to(index))
                    .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
            )
            .service(
                web::resource("/signup")
                    .route(web::post().to(sign_up))
                    .route(web::get().to(geti_id))
                    .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
            )
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("0.0.0.0:8000")?
    };

    server.run().await
}
