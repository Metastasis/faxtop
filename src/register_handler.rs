use crate::{email_service, error::AuthError, session::Confirmation, utils};
use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterData {
    pub email: String,
}

pub async fn send_confirmation(
    session: Session,
    data: web::Json<RegisterData>,
    db: web::Data<mongodb::Database>,
) -> Result<HttpResponse, AuthError> {
    if utils::is_signed_in(&session) {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let result = create_confirmation(data.into_inner().email, &db).await;
    match result {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Err(e),
    }
}

async fn create_confirmation(
    email: String,
    db: &web::Data<mongodb::Database>,
) -> Result<(), AuthError> {
    let confirmation = insert_record(email, db).await?;
    email_service::send_confirmation_mail(&confirmation)
}

async fn insert_record(
    email: String,
    db: &web::Data<mongodb::Database>,
) -> Result<Confirmation, AuthError> {
    let err = AuthError::GenericError("Cant save confirmation".to_string());
    let new_record: Confirmation = email.into();
    let result = new_record.clone();
    db.collection("confirmations")
        .insert_one(new_record.into(), None)
        .await
        .map_or(Err(err), move |_t| Ok(result))
}
