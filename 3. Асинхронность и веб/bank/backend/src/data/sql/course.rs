use crate::{
    domain::course::{self, Course, CourseRepository},
    infrastructure::error::ErrorApi,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_json::from_value;
use sqlx::PgPool;
use std::{collections::HashMap, sync::Arc};

#[derive(sqlx::FromRow)]
struct CourseRow {
    time_update_utc: DateTime<Utc>,
    base_code: String,
    conversion_rates: serde_json::Value,
}

impl TryFrom<CourseRow> for Course {
    type Error = ErrorApi;

    fn try_from(row: CourseRow) -> Result<Self, Self::Error> {
        let conversion_rates = from_value::<HashMap<String, f64>>(row.conversion_rates.clone())
            .map_err(|_| ErrorApi::Inner("Error parse conversion_rates".to_string()))?;

        let token = course::get_token();
        Ok(Course::new(
            token,
            row.time_update_utc,
            row.base_code,
            conversion_rates,
        ))
    }
}

pub struct CourseSQLRepo(pub Arc<PgPool>);

#[async_trait]
impl CourseRepository for CourseSQLRepo {
    async fn create(
        &mut self,
        time_update_utc: DateTime<Utc>,
        base_code: String,
        conversion_rates: HashMap<String, f64>,
    ) -> Result<Course, ErrorApi> {
        let json =
            serde_json::to_value(conversion_rates).map_err(|e| ErrorApi::Inner(e.to_string()))?;
        let row = sqlx::query_as!(
            CourseRow,
            r#"
            INSERT INTO courses (time_update_utc, base_code, conversion_rates)
            VALUES ($1, $2, $3)
            RETURNING time_update_utc, base_code, conversion_rates
            "#,
            time_update_utc,
            base_code,
            json
        )
        .fetch_one(self.0.as_ref())
        .await
        .map_err(|e| ErrorApi::DataBase(e.to_string()))?;

        Course::try_from(row)
    }
    async fn get_by_time(&self, time_update_utc: DateTime<Utc>) -> Option<Course> {
        let from = time_update_utc - chrono::Duration::hours(12);
        let to = time_update_utc + chrono::Duration::hours(12);

        let Ok(row) = sqlx::query_as!(
            CourseRow,
            r#"
            SELECT time_update_utc, base_code, conversion_rates
            FROM courses
            WHERE time_update_utc BETWEEN $1 AND $2
            ORDER BY ABS(EXTRACT(EPOCH FROM (time_update_utc - $3)))
            LIMIT 1
            "#,
            from,
            to,
            time_update_utc,
        )
        .fetch_optional(self.0.as_ref())
        .await
        else {
            return None;
        };

        Course::try_from(row?).ok()
    }
}
