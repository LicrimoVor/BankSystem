mod grpc;
mod http;
pub mod types;
pub mod dto {
    tonic::include_proto!("dto");
}
pub use grpc::GrpcClient;
pub use http::HttpClient;
use serde::Deserialize;

impl<'de> serde::Deserialize<'de> for dto::User {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        /// костыль)
        /// не хочется дублировать dto::*
        /// но наверное так надо сделать .-.
        #[derive(serde::Deserialize)]
        struct Helper {
            username: String,
            email: String,
        }

        let helper = Helper::deserialize(deserializer)?;
        Ok(dto::User {
            username: helper.username.into(),
            email: helper.email.into(),
        })
    }
}

impl<'de> serde::Deserialize<'de> for dto::Post {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            id: String,
            title: String,
            content: String,
            updated_at: String,
            author: Option<dto::User>,
            img_path: Option<String>,
        }

        let helper = Helper::deserialize(deserializer)?;
        Ok(dto::Post {
            id: helper.id.into(),
            title: helper.title.into(),
            content: helper.content.into(),
            updated_at: helper.updated_at.into(),
            author: helper.author,
            img_path: helper.img_path.map(Into::into),
        })
    }
}
