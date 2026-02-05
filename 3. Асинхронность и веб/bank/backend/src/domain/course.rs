use std::collections::HashMap;

use crate::{impl_constructor, infrastructure::error::ErrorApi};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use getset::Getters;
use serde::Serialize;

#[derive(Debug, Serialize, Getters, Clone)]
pub struct Course {
    #[getset(get = "pub")]
    time_update_utc: DateTime<Utc>,

    #[getset(get = "pub")]
    base_code: String,

    #[getset(get = "pub")]
    conversion_rates: HashMap<String, f64>,
}

#[async_trait]
pub trait CourseRepository: Send + Sync {
    async fn create(
        &mut self,
        time_update_utc: DateTime<Utc>,
        base_code: String,
        conversion_rates: HashMap<String, f64>,
    ) -> Result<Course, ErrorApi>;
    async fn get_by_time(&self, time_update_utc: DateTime<Utc>) -> Option<Course>;
}

impl_constructor!(token: CourseToken, Course, (
    time_update_utc: DateTime<Utc>,
    base_code: String,
    conversion_rates: HashMap<String, f64>
));
impl_constructor!(factory: Course, (
    time_update_utc: DateTime<Utc>,
    base_code: String,
    conversion_rates: HashMap<String, f64>
));
