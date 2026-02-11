use chrono::FixedOffset;

pub mod config;
pub mod database;
pub mod errors;
pub mod logging;
pub mod migrations;
pub mod security;
pub mod state;

pub const DATETIME_OFFSET: FixedOffset =
    FixedOffset::east_opt(8 * 3600).expect("FixedOffset::east out of bounds");
