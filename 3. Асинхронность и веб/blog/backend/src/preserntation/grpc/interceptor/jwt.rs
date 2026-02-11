use crate::{domain::auth::JwtToken, preserntation::grpc::consts::HEADER_AUTHORIZATION};
use tonic::{Request, Status};

pub fn jwt_interceptor(mut req: Request<()>) -> Result<Request<()>, Status> {
    let metadata = req.metadata();

    if let Some(jwt_header) = metadata.get(HEADER_AUTHORIZATION).cloned() {
        let jwt_header = jwt_header.to_str().unwrap();
        if let Some(jwt_token) = jwt_header.split(" ").last() {
            req.extensions_mut()
                .insert(JwtToken::from(jwt_token.to_string()));
        }
    }
    Ok(req)
}
