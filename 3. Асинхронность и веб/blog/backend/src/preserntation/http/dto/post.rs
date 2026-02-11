use super::user::UserResponse;
use crate::domain::{post::Post, user::User};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PostResponse {
    pub title: String,
    pub content: String,
    pub updated_at: String,
    pub img_path: Option<String>,
    pub author: UserResponse,
}

impl PostResponse {
    pub fn new(user: User, post: Post) -> PostResponse {
        Self {
            title: post.title().clone(),
            content: post.content().clone(),
            img_path: post.img_path().clone(),
            updated_at: post.updated_at().to_string(),
            author: UserResponse::new(user),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PostUpdate {
    pub title: Option<String>,
    pub content: Option<String>,
    pub img_base64: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PostCreate {
    pub title: String,
    pub content: String,
    pub img_base64: Option<String>,
}
