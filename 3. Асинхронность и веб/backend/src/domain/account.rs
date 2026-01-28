use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Account {
    pub id: Uuid,
    pub user_id: Uuid,
    balance: f64,
}

impl Account {}
