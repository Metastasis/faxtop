use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use mongodb::error::Error as DBError;
use std::convert::From;
use uuid::Error as UuidError;

#[derive(Debug, Display, Clone)]
pub enum AuthError {
    #[display(fmt = "BadId")]
    BadId,
    #[display(fmt = "GenericError: {}", _0)]
    GenericError(String),
    #[display(fmt = "ProcessFailed: {}", _0)]
    ProcessError(String),
    #[display(fmt = "AuthenticationError: {}", _0)]
    AuthenticationError(String),
    #[display(fmt = "NotFound: {}", _0)]
    NotFound(String),
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthError::BadId => HttpResponse::BadRequest().json("Invalid ID"),
            AuthError::GenericError(ref message) => HttpResponse::BadRequest().json(message),
            AuthError::ProcessError(ref message) => {
                HttpResponse::InternalServerError().json(message)
            }
            AuthError::AuthenticationError(ref message) => {
                HttpResponse::Unauthorized().json(message)
            }
            AuthError::NotFound(ref message) => HttpResponse::NotFound().json(message),
        }
    }
}

impl From<UuidError> for AuthError {
    fn from(_: UuidError) -> AuthError {
        AuthError::BadId
    }
}

impl From<DBError> for AuthError {
    fn from(error: DBError) -> AuthError {
        match error {
            _ => AuthError::GenericError(String::from("Some database error occured")),
        }
    }
}
