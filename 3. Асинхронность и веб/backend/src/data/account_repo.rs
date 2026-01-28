use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::{
    domain::user::User,
    infrastructure::{error::ErrorApi, state::State},
};

pub async fn create_account(
    state: &mut State,
    id: Uuid,
    email: &str,
    password_hash: &str,
) -> Result<(), ErrorApi> {
    let mut users = state.users().await;
    if users.values().find(|u| u.email == email).is_some() {
        return Err(ErrorApi::State("User already exists".to_string()));
    };
    let user = User {
        id,
        email: email.to_string(),
        password_hash: password_hash.to_string(),
        created_at: Utc::now(),
    };
    users.insert(id, user);

    Ok(())
}

pub async fn get_account_by_id(state: &State, email: &str) -> Option<User> {
    let users = state.users().await;
    users.values().find(|u| u.email == email).cloned()
}
