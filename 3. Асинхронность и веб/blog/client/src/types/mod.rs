pub mod auth;
pub mod general;
pub mod post;
pub mod user;

use tonic::Status;

#[derive(Debug)]
pub enum Error {
    Unauthenticated,
    Reqwest(reqwest::Error),
    Inner(String),
    Grps(Status),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Error::Inner(e)
    }
}

impl From<Status> for Error {
    fn from(e: Status) -> Self {
        Error::Grps(e)
    }
}

pub trait Client {
    fn auth(&self) -> Box<dyn auth::AuthClientTrait>;
    fn general(&self) -> Box<dyn general::GeneralClientTrait>;
    fn post(&self) -> Box<dyn post::PostClientTrait>;
    fn user(&self) -> Box<dyn user::UserClientTrait>;
}
