use regex::Regex;
use uuid::Uuid;

use crate::{
    data::{state::user::UserStateRepo, Database},
    domain::user::{User, UserRepository},
    infrastructure::error::ErrorApi,
};

const EMAIL_REGEX: &str = r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,}$";
const MIN_PASSWORD_LEN: usize = 8;

pub async fn create_user(db: Database, email: String, password: String) -> Result<User, ErrorApi> {
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

    let password_hash = password + "salt";

    let mut repo = match db {
        Database::STATE(state) => UserStateRepo(state),
        Database::SEA(_) => todo!(),
    };

    repo.create(email, password_hash).await
}

pub async fn delete_user(db: Database, user: &User) -> Result<(), ErrorApi> {
    let mut repo = match db {
        Database::STATE(state) => UserStateRepo(state),
        Database::SEA(_) => todo!(),
    };

    repo.delete(user).await
}

pub async fn get_user_by_id(db: Database, id: Uuid) -> Option<User> {
    let repo = match db {
        Database::STATE(state) => UserStateRepo(state),
        Database::SEA(_) => todo!(),
    };

    repo.get_by_id(id).await
}

pub async fn get_user_by_email(db: Database, email: String) -> Option<User> {
    let repo = match db {
        Database::STATE(state) => UserStateRepo(state),
        Database::SEA(_) => todo!(),
    };

    repo.get_by_email(email).await
}
