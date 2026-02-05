use crate::{
    domain::course::{self, Course, CourseRepository},
    infrastructure::{error::ErrorApi, state::State},
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::{collections::HashMap, sync::Arc};

pub struct CourseStateRepo(pub Arc<State>);

#[async_trait]
impl CourseRepository for CourseStateRepo {
    async fn create(
        &mut self,
        time_update_utc: DateTime<Utc>,
        base_code: String,
        conversion_rates: HashMap<String, f64>,
    ) -> Result<Course, ErrorApi> {
        let course = course::factory::create(time_update_utc.clone(), base_code, conversion_rates)?;
        let mut courses = self.0.courses().await;
        courses.insert(time_update_utc, course.clone());
        Ok(course)
    }
    async fn get_by_time(&self, time_update_utc: DateTime<Utc>) -> Option<Course> {
        let courses = self.0.courses().await;
        courses.get(&time_update_utc).cloned()
    }
}
