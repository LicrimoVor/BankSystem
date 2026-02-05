use crate::{
    data::Database,
    domain::course::Course,
    infrastructure::{config::Config, error::ErrorApi},
};
use chrono::DateTime;
use reqwest::Client;
use serde_json::from_value;
use std::{collections::HashMap, sync::Arc};
use tracing::{debug, info};

pub async fn get_course(
    db: Arc<Database>,
    client: Arc<Client>,
    config: Arc<Config>,
) -> Result<Course, ErrorApi> {
    let mut course_repo = db.get_course_repo();
    let now = chrono::Utc::now();
    if let Some(course) = course_repo.get_by_time(now).await {
        debug!("Course get from database");
        return Ok(course);
    };

    debug!("Get course from api");

    let res = client
        .get(format!(
            "https://v6.exchangerate-api.com/v6/{}/latest/USD",
            config.api_key
        ))
        .send()
        .await
        .map_err(|_| ErrorApi::Inner("Error get courses".to_string()))?;

    let data: serde_json::Value = res
        .json()
        .await
        .map_err(|_| ErrorApi::Inner("Eror parse course".to_string()))?;
    let Some(time_last_update_unix) = data["time_last_update_unix"].as_i64() else {
        return Err(ErrorApi::Inner(
            "Error parse time_last_update_unix".to_string(),
        ));
    };
    let Some(base_code) = data["base_code"].as_str() else {
        return Err(ErrorApi::Inner("Error parse base_code".to_string()));
    };

    let conversion_rates = from_value::<HashMap<String, f64>>(data["conversion_rates"].clone())
        .map_err(|_| ErrorApi::Inner("Error parse conversion_rates".to_string()))?;

    let Some(time_last_update_unix) = DateTime::from_timestamp_secs(time_last_update_unix) else {
        return Err(ErrorApi::Inner(
            "Error parse time_last_update_unix".to_string(),
        ));
    };

    let course = course_repo
        .create(
            time_last_update_unix,
            base_code.to_string(),
            conversion_rates.clone().into(),
        )
        .await?;

    Ok(course)
}
