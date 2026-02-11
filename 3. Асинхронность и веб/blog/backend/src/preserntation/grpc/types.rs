use crate::{domain, preserntation::grpc::interceptor::time::TimeLayer};
use tonic::{Request, Response, Status, service::InterceptorLayer, transport::server::Router};
use tower::layer::util::{Identity, Stack};
use uuid::Uuid;

pub(super) mod auth_service {
    tonic::include_proto!("auth");
}
pub(super) mod general_service {
    tonic::include_proto!("general");
}
pub(super) mod post_service {
    tonic::include_proto!("post");
}
pub(super) mod user_service {
    tonic::include_proto!("user");
}
pub(super) mod dto {
    tonic::include_proto!("dto");
}

pub(super) type ResultService<T> = Result<Response<T>, Status>;
#[derive(Debug, Clone)]
pub(super) struct RequestId(pub Uuid);

pub(super) type InterceptorFn = fn(Request<()>) -> Result<Request<()>, Status>;
pub(super) type RouterType = Router<
    Stack<
        Stack<
            InterceptorLayer<InterceptorFn>,
            Stack<TimeLayer, Stack<InterceptorLayer<InterceptorFn>, Identity>>,
        >,
        Identity,
    >,
>;

impl From<domain::user::User> for dto::User {
    fn from(user: domain::user::User) -> Self {
        Self {
            username: user.username().clone(),
            email: user.email().clone(),
        }
    }
}

impl From<(domain::user::User, domain::post::Post)> for dto::Post {
    fn from((author, post): (domain::user::User, domain::post::Post)) -> Self {
        Self {
            id: post.id().to_string(),
            title: post.title().clone(),
            content: post.content().clone(),
            img_path: post.img_path().clone(),
            updated_at: post.updated_at().to_string(),
            author: Some(author.into()),
        }
    }
}
