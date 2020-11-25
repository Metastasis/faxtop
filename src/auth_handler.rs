use crate::error::AuthError;
use crate::session::SessionUser;
use crate::utils::{get_current_user, is_signed_in, set_current_user, verify};
use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};
use mongodb::{
    bson::{doc, Bson},
    Database,
};
use serde::Deserialize;
use std::str::FromStr;
use uuid::Uuid;

pub async fn me(session: Session, _req: HttpRequest) -> HttpResponse {
    let user_result = dbg!(get_current_user(&session));
    user_result.map_or(HttpResponse::Unauthorized().json("Unauthorized"), |user| {
        HttpResponse::Ok().json(user)
    })
}

pub async fn sign_out(session: Session) -> HttpResponse {
    session.clear();
    HttpResponse::NoContent().finish()
}

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

pub async fn sign_in(
    data: web::Json<AuthData>,
    session: Session,
    req: HttpRequest,
    pool: web::Data<Database>,
) -> Result<HttpResponse, AuthError> {
    match is_signed_in(&session) {
        true => {
            let response = get_current_user(&session)
                .map(|user| HttpResponse::Ok().json(user))
                .unwrap();
            Ok(response)
        }
        false => handle_sign_in(data.into_inner(), &session, &req, &pool).await,
    }
}

async fn handle_sign_in(
    data: AuthData,
    session: &Session,
    _req: &HttpRequest,
    pool: &web::Data<Database>,
) -> Result<HttpResponse, AuthError> {
    let result = find_user(data, pool).await;
    match result {
        Ok(user) => {
            set_current_user(&session, &user);
            Ok(HttpResponse::Ok().json(user))
        }
        Err(err) => Ok(HttpResponse::Unauthorized().json(err.to_string())),
    }
}

async fn find_user(data: AuthData, db: &web::Data<Database>) -> Result<SessionUser, AuthError> {
    let result = db
        .collection("users")
        .find_one(doc! {"email": (data.email.to_string())}, None)
        .await;
    if result.is_err() {
        return Err(AuthError::NotFound(String::from("Confirmation not found")));
    }
    let result = result.unwrap();
    if result.is_none() {
        return Err(AuthError::NotFound(String::from("Confirmation not found")));
    }
    let document = result.unwrap();
    let id = document.get("id").and_then(Bson::as_str);
    let email = document.get("email").and_then(Bson::as_str);
    let hash = document.get("hash").and_then(Bson::as_str);
    if hash.is_none() || id.is_none() || email.is_none() {
        return Err(AuthError::GenericError(String::from("Bad request")));
    }
    if let Ok(matching) = verify(&hash.unwrap(), &data.password) {
        if matching == () {
            return Ok(SessionUser {
                id: Uuid::from_str(id.unwrap()).unwrap(),
                email: email.unwrap().to_string(),
            });
        }
    }
    Err(AuthError::NotFound(String::from("User not found")))
}
