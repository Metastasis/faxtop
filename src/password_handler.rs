use crate::{
    error::AuthError,
    session::{Confirmation, SessionUser, User},
    utils::{hash_password, is_signed_in, set_current_user},
};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use mongodb::{
    bson::{doc, Bson},
    Database,
};
use serde::Deserialize;
use std::str::FromStr;
use uuid::Uuid;
use chrono::{Utc, NaiveDateTime};

#[derive(Debug, Deserialize)]
pub struct PasswordData {
    pub password: String,
}

pub async fn create_account(
    session: Session,
    path_id: web::Path<String>,
    data: web::Json<PasswordData>,
    db: web::Data<Database>,
) -> Result<HttpResponse, AuthError> {
    if is_signed_in(&session) {
        return Ok(HttpResponse::BadRequest().finish());
    }
    let result = create_user(&path_id.into_inner(), &data.into_inner().password, &db).await;
    match result {
        Ok(user) => {
            set_current_user(&session, &user);
            Ok(HttpResponse::Created().json(&user))
        }
        Err(err) => Err(err),
    }
}

async fn create_user(
    path_id: &str,
    password: &str,
    db: &web::Data<Database>,
) -> Result<SessionUser, AuthError> {
    let path_uuid = uuid::Uuid::parse_str(path_id)?;
    let result = db
        .collection("confirmations")
        .find_one(doc! {"id": (path_uuid.to_string())}, None)
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
    let expires_at = document.get("expiresAt").and_then(Bson::as_i64);
    if id.is_none() || email.is_none() || expires_at.is_none() {
        return Err(AuthError::GenericError(String::from("Bad request")));
    }
    let at = NaiveDateTime::from_timestamp(expires_at.unwrap(), 0);
    let confirmation = Confirmation {
        id: Uuid::from_str(id.unwrap()).unwrap(),
        email: email.unwrap().to_string(),
        expires_at: chrono::DateTime::from_utc(at, Utc),
    };
    if confirmation.expires_at > chrono::Utc::now() {
        let password: String = hash_password(password)?;
        let user = User::from(confirmation.email, password);
        let res = db
            .collection("users")
            .insert_one(user.clone().into(), None)
            .await;
        return match res {
            Ok(_) => Ok(user.into()),
            Err(_) => Err(AuthError::GenericError(String::from("Bad request"))),
        };
    }
    Err(AuthError::GenericError(String::from("Bad request")))
}
