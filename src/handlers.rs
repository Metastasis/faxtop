use actix_web::Responder;

pub async fn get_crawlers() -> impl Responder {
    format!("Response from get_crawlers")
}

pub async fn get_crawler_by_id() -> impl Responder {
    format!("Response from get_crawler_by_id")
}

pub async fn add_crawler() -> impl Responder {
    format!("Response from add_crawler")
}
