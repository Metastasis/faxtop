use crate::{error, session};

pub fn hash_password(password: &str) -> Result<String, error::AuthError> {
    pbkdf2::pbkdf2_simple(password, 32)
        .map_err(|_| error::AuthError::AuthenticationError(String::from("Could not hash password")))
}

pub fn verify(hash: &str, password: &str) -> Result<(), error::AuthError> {
    pbkdf2::pbkdf2_check(password, hash).map_err(|_| {
        error::AuthError::AuthenticationError(String::from("Could not verify password"))
    })
}

pub fn is_signed_in(session: &actix_session::Session) -> bool {
    let result = get_current_user(session);
    match result {
        Ok(_) => true,
        _ => false,
    }
}

pub fn set_current_user(session: &actix_session::Session, user: &session::SessionUser) {
    // serializing to string is alright for this case,
    // but binary would be preferred in production use-cases.
    session
        .set("user", serde_json::to_string(user).unwrap())
        .unwrap();
}

pub fn get_current_user(
    session: &actix_session::Session,
) -> Result<session::SessionUser, error::AuthError> {
    let err =
        error::AuthError::AuthenticationError(String::from("Could not retrieve user from session"));
    let session_result = session.get::<String>("user"); // Returns Result<Option<String>, Error>
    if session_result.is_err() {
        return Err(err);
    }
    session_result
        .unwrap()
        .map_or(Err(err.clone()), |user_str| {
            serde_json::from_str(&user_str).or_else(|_| Err(err))
        })
}
