use super::super::{dto, post_service::*};
use crate::{
    application::{post::PostService, user::UserService},
    data::Database,
    infrastructure::{config::Config, errors::ErrorBlog},
    preserntation::grpc::{ResultService, extractor::extract_user_id},
};
use std::{str::FromStr, sync::Arc};
use tonic::{Request, Response};
use uuid::Uuid;

pub struct PostGRPCSerivce {
    pub database: Arc<Database>,
    pub config: Arc<Config>,
}

#[tonic::async_trait]
impl post_service_server::PostService for PostGRPCSerivce {
    async fn create_post(&self, request: Request<PostCreateRequest>) -> ResultService<dto::Post> {
        let user_id = extract_user_id(&self.config, &request)?;
        let post_service = PostService(self.database.clone());
        let user_service = UserService(self.database.clone());
        let PostCreateRequest {
            title,
            content,
            img_base64,
        } = request.into_inner();
        let user = user_service.get_by_id(user_id).await?;
        let post = post_service
            .create(self.config.clone(), title, content, user_id, img_base64)
            .await?;
        Ok(Response::new((user, post).into()))
    }
    async fn gets_by_author(
        &self,
        request: Request<GetByAuthorPostRequest>,
    ) -> ResultService<PostsResponse> {
        let post_service = PostService(self.database.clone());
        let user_service = UserService(self.database.clone());
        let GetByAuthorPostRequest { email } = request.into_inner();
        let user = user_service.get_by_email(email).await?;
        let posts = post_service.gets_by_author(*user.id()).await?;
        let data: Vec<dto::Post> = posts
            .into_iter()
            .map(|post| (user.clone(), post).into())
            .collect();
        Ok(Response::new(PostsResponse { posts: data }))
    }

    async fn gets_me(&self, request: Request<dto::Empty>) -> ResultService<PostsResponse> {
        let user_id = extract_user_id(&self.config, &request)?;
        let post_service = PostService(self.database.clone());
        let user_service = UserService(self.database.clone());
        let user = user_service.get_by_id(user_id).await?;
        let posts = post_service.gets_by_author(user_id).await?;
        let data: Vec<dto::Post> = posts
            .into_iter()
            .map(|post| (user.clone(), post).into())
            .collect();
        Ok(Response::new(PostsResponse { posts: data }))
    }
    async fn update_post(&self, request: Request<PostUpdateRequest>) -> ResultService<dto::Post> {
        let user_id = extract_user_id(&self.config, &request)?;
        let post_service = PostService(self.database.clone());
        let user_service = UserService(self.database.clone());
        let PostUpdateRequest {
            id,
            title,
            content,
            img_base64,
        } = request.into_inner();
        let post_id = Uuid::from_str(id.as_str())
            .map_err(|_| ErrorBlog::Validation("Failed parse post id".to_string()))?;
        let user = user_service.get_by_id(user_id).await?;
        let post = post_service
            .update(
                self.config.clone(),
                post_id,
                user_id,
                title,
                content,
                img_base64,
            )
            .await?;
        Ok(Response::new((user, post).into()))
    }

    async fn delete_post(&self, request: Request<PostDeleteRequest>) -> ResultService<dto::Empty> {
        let user_id = extract_user_id(&self.config, &request)?;
        let post_service = PostService(self.database.clone());
        let PostDeleteRequest { id } = request.into_inner();
        let post_id = Uuid::from_str(id.as_str())
            .map_err(|_| ErrorBlog::Validation("Failed parse post id".to_string()))?;
        post_service.delete(user_id, post_id).await?;
        Ok(dto::Empty {}.into())
    }
    async fn get_by_id(&self, request: Request<GetPostRequest>) -> ResultService<dto::Post> {
        let post_service = PostService(self.database.clone());
        let user_service = UserService(self.database.clone());
        let GetPostRequest { id } = request.into_inner();
        let post_id = Uuid::from_str(id.as_str())
            .map_err(|_| ErrorBlog::Validation("Failed parse post id".to_string()))?;
        let post = post_service.get_by_id(post_id).await?;
        let user = user_service.get_by_id(*post.author_id()).await?;
        Ok(Response::new((user, post).into()))
    }
}

pub fn init(
    database: Arc<Database>,
    config: Arc<Config>,
) -> post_service_server::PostServiceServer<PostGRPCSerivce> {
    post_service_server::PostServiceServer::new(PostGRPCSerivce { database, config })
}
