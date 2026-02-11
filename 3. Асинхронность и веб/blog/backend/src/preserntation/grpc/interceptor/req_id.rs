use super::super::RequestId;
use tonic::{Request, Status};
use uuid::Uuid;

pub fn req_id_interceptor(mut req: Request<()>) -> Result<Request<()>, Status> {
    let id = Uuid::new_v4();
    req.extensions_mut().insert(RequestId(id));
    Ok(req)
}
