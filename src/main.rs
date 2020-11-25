mod auth_handler;
mod email_service;
mod error;
mod handlers;
mod password_handler;
mod register_handler;
mod session;
mod utils;
mod vars;

extern crate log;

use actix_web::{web, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "info,debug,error,warn");
    env_logger::init();
    let client_options = mongodb::options::ClientOptions::parse(vars::database_url().as_str())
        .await
        .unwrap();
    let client = mongodb::Client::with_options(client_options.clone()).unwrap();
    let db = client.database("faxtop");
    HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(
                actix_session::CookieSession::signed(&[0; 32])
                    // .domain(vars::domain_url().as_str())
                    .name("auth")
                    .secure(false),
            )
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allowed_methods(vec!["GET", "POST"])
                    // .allowed_headers(vec![http::header::ACCEPT, http::header::CONTENT_TYPE])
                    .max_age(3600),
            )
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/register")
                            .route(web::post().to(register_handler::send_confirmation)),
                    )
                    .service(
                        web::resource("/register/{path_id}")
                            .route(web::post().to(password_handler::create_account)),
                    )
                    .route("/me", web::get().to(auth_handler::me))
                    .route("/signout", web::get().to(auth_handler::sign_out))
                    .route("/signin", web::post().to(auth_handler::sign_in)),
            )
            .route("/crawlers", web::get().to(handlers::get_crawlers))
            .route("/crawlers", web::post().to(handlers::add_crawler))
            .route("/crawlers/{id}", web::get().to(handlers::get_crawler_by_id))
    })
    .bind(format!("{}:{}", vars::host(), vars::port()))?
    .run()
    .await
}
