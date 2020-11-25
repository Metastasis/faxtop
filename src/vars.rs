use std::env::var;

pub fn database_url() -> String {
    var("FAXTOP_MONGO_URL").expect("FAXTOP_MONGO_URL is not set")
}

pub fn domain_url() -> String {
    var("FAXTOP_DOMAIN_URL").expect("FAXTOP_DOMAIN_URL is not set")
}

pub fn host() -> String {
    var("FAXTOP_HOST").unwrap_or(String::from("localhost"))
}

pub fn port() -> String {
    var("FAXTOP_PORT").unwrap_or(String::from("3000"))
}

// pub fn secret_key() -> String {
//     dotenv().ok();
//     var("FAXTOP_SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8))
// }

pub fn smtp_username() -> String {
    var("FAXTOP_SMTP_USERNAME").expect("FAXTOP_SMTP_USERNAME is not set")
}

pub fn smtp_password() -> String {
    var("FAXTOP_SMTP_PASSWORD").expect("FAXTOP_SMTP_PASSWORD is not set")
}

pub fn smtp_host() -> String {
    var("FAXTOP_SMTP_HOST").expect("FAXTOP_SMTP_HOST is not set")
}

pub fn smtp_port() -> u16 {
    var("FAXTOP_SMTP_PORT")
        .expect("FAXTOP_SMTP_PORT is not set")
        .parse::<u16>()
        .ok()
        .expect("SMTP_PORT should be an integer")
}

#[allow(dead_code)]
pub fn smtp_sender_name() -> String {
    var("SMTP_SENDER_NAME").expect("SMTP_SENDER_NAME is not set")
}
