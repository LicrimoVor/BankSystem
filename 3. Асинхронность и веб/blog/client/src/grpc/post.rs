use super::proto::post::*;
use crate::{
    dto,
    grpc::{GrpcState, utils::auth_request},
};
use std::sync::{Arc, Mutex};
use tonic::{Request, Status};

pub struct PostClient {
    inner: post_service_client::PostServiceClient<tonic::transport::Channel>,
    state: Arc<Mutex<GrpcState>>,
}

impl PostClient {
    pub(super) fn new(channel: tonic::transport::Channel, state: Arc<Mutex<GrpcState>>) -> Self {
        Self {
            inner: post_service_client::PostServiceClient::new(channel),
            state,
        }
    }

    pub async fn create(
        &mut self,
        title: String,
        content: String,
        img_base64: Option<String>,
    ) -> Result<dto::Post, Status> {
        let Some(jwt_token) = self.state.lock().unwrap().access_token.clone() else {
            return Err(Status::unauthenticated("unauthorized"));
        };

        let req = PostCreateRequest {
            title,
            content,
            img_base64,
        };
        Ok(self
            .inner
            .create_post(auth_request(req, jwt_token))
            .await?
            .into_inner())
    }

    pub async fn update(
        &mut self,
        id: String,
        title: Option<String>,
        content: Option<String>,
        img_base64: Option<String>,
    ) -> Result<dto::Post, Status> {
        let Some(jwt_token) = self.state.lock().unwrap().access_token.clone() else {
            return Err(Status::unauthenticated("unauthorized"));
        };
        let req = PostUpdateRequest {
            id,
            title,
            content,
            img_base64,
        };
        Ok(self
            .inner
            .update_post(auth_request(req, jwt_token))
            .await?
            .into_inner())
    }

    pub async fn delete(&mut self, id: String) -> Result<dto::Empty, Status> {
        let Some(jwt_token) = self.state.lock().unwrap().access_token.clone() else {
            return Err(Status::unauthenticated("unauthorized"));
        };
        Ok(self
            .inner
            .delete_post(auth_request(PostDeleteRequest { id }, jwt_token))
            .await?
            .into_inner())
    }

    pub async fn gets_me(&mut self) -> Result<Vec<dto::Post>, Status> {
        let Some(jwt_token) = self.state.lock().unwrap().access_token.clone() else {
            return Err(Status::unauthenticated("unauthorized"));
        };
        let PostsResponse { posts } = self
            .inner
            .gets_me(auth_request(dto::Empty {}, jwt_token))
            .await?
            .into_inner();
        Ok(posts)
    }

    pub async fn get_by_id(&mut self, id: String) -> Result<dto::Post, Status> {
        Ok(self
            .inner
            .get_by_id(Request::new(GetPostRequest { id }))
            .await?
            .into_inner())
    }

    pub async fn gets_by_author(&mut self, email: String) -> Result<Vec<dto::Post>, Status> {
        let PostsResponse { posts } = self
            .inner
            .gets_by_author(Request::new(GetByAuthorPostRequest { email }))
            .await?
            .into_inner();
        Ok(posts)
    }
}
