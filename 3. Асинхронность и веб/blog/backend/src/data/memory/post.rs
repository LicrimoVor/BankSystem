use crate::{
    domain::post::{Post, PostRepository, factory},
    infrastructure::{errors::ErrorBlog, state::State},
};
use std::sync::Arc;
use uuid::Uuid;

pub struct PostStateRepo(pub Arc<State>);

#[async_trait::async_trait]
impl PostRepository for PostStateRepo {
    async fn create(
        &self,
        title: String,
        content: String,
        author_id: Uuid,
        img_path: Option<String>,
    ) -> Result<Post, ErrorBlog> {
        let post_state = &mut self.0.get_mut_posts().await;
        let post = factory::create(title, content, author_id, img_path)?;
        post_state
            .entry(author_id)
            .or_default()
            .insert(post.id().clone(), post.clone());
        Ok(post)
    }

    async fn update(&self, post_id: Uuid, post: Post) -> Result<Post, ErrorBlog> {
        let post_state = &mut self.0.get_mut_posts().await;
        if let Some(author_posts) = post_state.get_mut(&post.author_id()) {
            if author_posts.contains_key(&post_id) {
                author_posts.insert(post_id, post.clone());
                return Ok(post);
            }
        }
        Err(ErrorBlog::NotFound(format!(
            "Post with id {} not found",
            post_id
        )))
    }

    async fn delete(&self, post_id: Uuid) -> Result<Post, ErrorBlog> {
        let post_state = &mut self.0.get_mut_posts().await;
        for author_posts in post_state.values_mut() {
            if let Some(post) = author_posts.remove(&post_id) {
                return Ok(post);
            }
        }
        Err(ErrorBlog::NotFound(format!(
            "Post with id {} not found",
            post_id
        )))
    }

    async fn get_by_id(&self, post_id: Uuid) -> Result<Option<Post>, ErrorBlog> {
        let post_state = &self.0.get_posts().await;
        for author_posts in post_state.values() {
            if let Some(post) = author_posts.get(&post_id) {
                return Ok(Some(post.clone()));
            }
        }
        Ok(None)
    }

    async fn gets_by_author(&self, author_id: Uuid) -> Result<Vec<Post>, ErrorBlog> {
        let post_state = &self.0.get_posts().await;
        if let Some(author_posts) = post_state.get(&author_id) {
            return Ok(author_posts.values().cloned().collect());
        }
        Ok(vec![])
    }
}
