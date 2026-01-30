use std::sync::Arc;

use regex::Regex;
use uuid::Uuid;

use crate::{
    data::Database,
    domain::user::User,
    infrastructure::{error::ErrorApi, security::hash_password},
};

const EMAIL_REGEX: &str = r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+.[A-Za-z]{2,}$";
const MIN_PASSWORD_LEN: usize = 8;

pub async fn create_user(
    db: Arc<Database>,
    email: String,
    password: String,
) -> Result<User, ErrorApi> {
    let email = email.trim().to_lowercase();
    let db = (db.clone()).clone();
    if password.len() < MIN_PASSWORD_LEN {
        return Err(ErrorApi::Validation(
            "Password must be at least 8 characters".to_string(),
        ));
    }
    let Ok(regex) = Regex::new(EMAIL_REGEX) else {
        return Err(ErrorApi::Inner("Invalid regex".to_string()));
    };
    if !regex.is_match(&email) {
        return Err(ErrorApi::Validation("Invalid email format".to_string()));
    };

    let Ok(password_hash) = hash_password(&password) else {
        return Err(ErrorApi::Inner("Hash error".to_string()));
    };
    let mut repo = db.get_user_repo();
    repo.create(email, password_hash).await
}

pub async fn delete_user(db: Arc<Database>, user: &User) -> Result<(), ErrorApi> {
    let mut repo = db.get_user_repo();
    repo.delete(user).await
}

pub async fn get_user_by_id(db: Arc<Database>, id: Uuid) -> Option<User> {
    let repo = db.get_user_repo();
    repo.get_by_id(id).await
}

pub async fn get_user_by_email(db: Arc<Database>, email: String) -> Option<User> {
    let repo = db.get_user_repo();
    repo.get_by_email(email).await
}
