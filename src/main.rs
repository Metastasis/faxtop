mod handlers;
mod vars;

extern crate log;

use actix_web::{web, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let client_options = mongodb::options::ClientOptions::parse(vars::database_url().as_str())
        .await
        .unwrap();
    let client = mongodb::Client::with_options(client_options).unwrap();
    let db = client.database("faxtop");
    for collection_name in db.list_collection_names(None).await.unwrap() {
        println!("{}", collection_name);
    }
    HttpServer::new(move || {
        App::new()
            .route("/crawlers", web::get().to(handlers::get_crawlers))
            .route("/crawlers", web::post().to(handlers::add_crawler))
            .route("/crawlers/{id}", web::get().to(handlers::get_crawler_by_id))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
