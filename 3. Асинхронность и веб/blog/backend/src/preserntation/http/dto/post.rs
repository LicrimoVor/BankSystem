use super::user::UserResponse;
use crate::domain::{post::Post, user::User};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct PostResponse {
    pub id: String,
    pub title: String,
    pub content: String,
    pub updated_at: String,
    pub img_path: Option<String>,
    pub author: UserResponse,
}

impl PostResponse {
    pub fn new(user: User, post: Post) -> PostResponse {
        Self {
            id: post.id().to_string(),
            title: post.title().clone(),
            content: post.content().clone(),
            img_path: post.img_path().clone(),
            updated_at: post.updated_at().to_string(),
            author: UserResponse::new(user),
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct PostUpdate {
    pub title: Option<String>,
    pub content: Option<String>,
    pub img_base64: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct PostCreate {
    pub title: String,
    pub content: String,
    pub img_base64: Option<String>,
}
