use dotenv::dotenv;
use std::env::var;

pub fn database_url() -> String {
    dotenv().ok();
    var("MONGO_URL").expect("DATABASE_URL is not set")
}
