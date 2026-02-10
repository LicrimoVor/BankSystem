use crate::infrastructure::{config::Config, errors::ErrorBlog};
use base64::{Engine, prelude::BASE64_STANDARD};
use std::sync::Arc;
use tokio::{fs::File, io::AsyncWriteExt};
use uuid::Uuid;

/// Сохранение изображения из строки base64 в файл
pub async fn save_image(config: Arc<Config>, image: String) -> Result<String, ErrorBlog> {
    let base64 = image
        .split(',')
        .last()
        .ok_or("Неверная строка base64")
        .map_err(|_| ErrorBlog::Validation("Неверная строка base64".to_string()))?;
    let bytes = BASE64_STANDARD
        .decode(base64)
        .map_err(|_| ErrorBlog::Validation("Неверная строка base64".to_string()))?;
    let img_path = format!("{}/{}.png", config.media_path, Uuid::new_v4());
    let mut file = File::create(img_path.clone())
        .await
        .map_err(|_| ErrorBlog::Internal("Ошибка создания файла".to_string()))?;
    file.write_all(&bytes)
        .await
        .map_err(|_| ErrorBlog::Internal("Ошибка записи файла".to_string()))?;
    return Ok(img_path);
}
