use std::str::FromStr;

use tonic::{Request, metadata::MetadataValue};

pub const HEADER_AUTHORIZATION: &'static str = "authorization";

pub fn auth_request<T>(data: T, jwt_token: String) -> Request<T> {
    let mut req = Request::new(data);
    req.metadata_mut().insert(
        HEADER_AUTHORIZATION,
        MetadataValue::from_str(format!("Bearer {}", jwt_token).as_str()).unwrap(),
    );
    req
}
