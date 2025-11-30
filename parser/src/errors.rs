/// # Ошибки парсинга
#[derive(Debug)]
pub enum ParseFileError {
    /// Ошибка чтения файла
    IoError(std::io::Error),

    /// Неверный формат строки
    ParseError(&'static str),

    /// Ошибка записи файла
    WriteError(std::io::Error),
}
