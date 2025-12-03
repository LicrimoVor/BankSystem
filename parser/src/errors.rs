use thiserror::Error;

/// ## Ошибки парсинга
///
/// ### Возможные ошибки
/// - [ParseFileError::IoError] - I/O ошибка
/// - [ParseFileError::SerializeError] - Неверный формат операций (entity -> file)
/// - [ParseFileError::DeSerializeError] - Неверный формат операций (file -> entity)
#[derive(Debug, Error)]
pub enum ParseFileError {
    #[error("I/O ошибка: {0}")]
    IoError(std::io::Error),

    #[error("Неверный формат: {0}")]
    SerializeError(&'static str),

    #[error("Неверный формат: {0}")]
    DeSerializeError(&'static str),
}
