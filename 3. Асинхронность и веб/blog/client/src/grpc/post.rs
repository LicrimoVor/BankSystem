use super::proto::post::*;
use crate::{
    dto,
    grpc::{GrpcState, utils::auth_request},
    types::{Error, post::PostClientTrait},
};
use std::sync::{Arc, Mutex};
use tonic::Request;

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
}

#[tonic::async_trait]
impl PostClientTrait for PostClient {
    async fn create(
        &mut self,
        title: &str,
        content: &str,
        img_base64: Option<&str>,
    ) -> Result<dto::Post, Error> {
        let Some(jwt_token) = self.state.lock().unwrap().access_token.clone() else {
            return Err(Error::Unauthenticated);
        };

        let req = PostCreateRequest {
            title: title.to_string(),
            content: content.to_string(),
            img_base64: img_base64.map(String::from),
        };
        Ok(self
            .inner
            .create_post(auth_request(req, jwt_token))
            .await?
            .into_inner())
    }

    async fn update(
        &mut self,
        id: &str,
        title: Option<&str>,
        content: Option<&str>,
        img_base64: Option<&str>,
    ) -> Result<dto::Post, Error> {
        let Some(jwt_token) = self.state.lock().unwrap().access_token.clone() else {
            return Err(Error::Unauthenticated);
        };
        let req = PostUpdateRequest {
            id: id.to_string(),
            title: title.map(String::from),
            content: content.map(String::from),
            img_base64: img_base64.map(String::from),
        };
        Ok(self
            .inner
            .update_post(auth_request(req, jwt_token))
            .await?
            .into_inner())
    }

    async fn delete(&mut self, id: &str) -> Result<(), Error> {
        let Some(jwt_token) = self.state.lock().unwrap().access_token.clone() else {
            return Err(Error::Unauthenticated);
        };
        self.inner
            .delete_post(auth_request(
                PostDeleteRequest { id: id.to_string() },
                jwt_token,
            ))
            .await?;
        Ok(())
    }

    async fn gets_me(&mut self) -> Result<Vec<dto::Post>, Error> {
        let Some(jwt_token) = self.state.lock().unwrap().access_token.clone() else {
            return Err(Error::Unauthenticated);
        };
        let PostsResponse { posts } = self
            .inner
            .gets_me(auth_request(dto::Empty {}, jwt_token))
            .await?
            .into_inner();
        Ok(posts)
    }

    async fn get_by_id(&mut self, id: &str) -> Result<dto::Post, Error> {
        Ok(self
            .inner
            .get_by_id(Request::new(GetPostRequest { id: id.to_string() }))
            .await?
            .into_inner())
    }

    async fn gets_by_author(&mut self, email: &str) -> Result<Vec<dto::Post>, Error> {
        let PostsResponse { posts } = self
            .inner
            .gets_by_author(Request::new(GetByAuthorPostRequest {
                email: email.to_string(),
            }))
            .await?
            .into_inner();
        Ok(posts)
    }
}
