use actix_cors::Cors;
use actix_web::{http, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;
use serde::{Deserialize, Serialize};

use env_logger::Env;
use std::collections::HashMap;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(Deserialize, Serialize)]
struct Creds {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct NewCreds {
    username: String,
    password: String,
    email: Option<String>,
}

async fn sign_in(creds: web::Json<Creds>) -> impl Responder {
    println!("{} {}", creds.username, creds.password);
    println!("success");
    HttpResponse::Ok().content_type("application/json").body(
        "
        { 'message' : 'received'}
        ",
    )
}

async fn sign_up(creds: web::Json<NewCreds>) -> impl Responder {
    println!("{:?}", creds);
    println!("success");
    HttpResponse::Ok().content_type("application/json").body(
        "
        { 'message' : 'received'}
        ",
    )
}

async fn greet() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        "
    <html>
        <body>
            <h1> Hello </h1>
        </body>
    </html>
    ",
    )
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::new() // <- Construct CORS middleware builder
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .finish(),
            )
            .wrap(Logger::default())
            .service(
                web::resource("/signin")
                    .route(web::post().to(sign_in))
                    .route(web::get().to(greet))
                    .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
            )
            .service(
                web::resource("/signup")
                    .route(web::post().to(sign_up))
                    .route(web::get().to(greet))
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
