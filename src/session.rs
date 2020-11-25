use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub hash: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionUser {
    pub id: Uuid,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Confirmation {
    pub id: Uuid,
    pub email: String,
    pub expires_at: chrono::NaiveDateTime,
}

impl From<User> for SessionUser {
    fn from(User { email, id, .. }: User) -> Self {
        SessionUser { email, id }
    }
}

impl From<Confirmation> for mongodb::bson::Document {
    fn from(confirmation: Confirmation) -> Self {
        doc!(
            "id": (confirmation.id.to_string()),
            "email": (confirmation.email),
            "expiresAt": (confirmation.expires_at.to_string())
        )
    }
}

impl User {
    pub fn from<S: Into<String>, T: Into<String>>(email: S, pwd: T) -> Self {
        User {
            id: Uuid::new_v4(),
            email: email.into(),
            hash: pwd.into(),
            created_at: chrono::Local::now().naive_local(),
        }
    }
}

impl From<User> for mongodb::bson::Document {
    fn from(user: User) -> Self {
        doc!(
            "id": (user.id.to_string()),
            "email": (user.email),
            "hash": (user.hash),
            "createdAt": (user.created_at.to_string()),
        )
    }
}

// any type that implements Into<String> can be used to create a Confirmation
impl<T> From<T> for Confirmation
where
    T: Into<String>,
{
    fn from(email: T) -> Self {
        Confirmation {
            id: Uuid::new_v4(),
            email: email.into(),
            expires_at: chrono::Local::now().naive_local() + chrono::Duration::hours(24),
        }
    }
}
