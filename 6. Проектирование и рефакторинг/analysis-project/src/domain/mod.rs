mod auth;
mod backet;
mod user;
mod utils;

use crate::parser::{Parsable, Parser};

pub fn just_parse<T: Parsable>(input: &str) -> Result<(&str, T), ()> {
    T::parser().parse(input)
}

pub use auth::AuthData;
pub use backet::{Announcements, AssetDsc, Backet};
pub use user::{UserBacket, UserBackets, UserCash};
