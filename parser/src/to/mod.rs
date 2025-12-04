mod bin;
mod csv;
mod txt;
use tracing::{error, instrument};

use crate::{OperationName, errors::ParseFileError, types::FileType};

/// ## Парсит файл в зависимости от его типа
/// (пока только операции)
pub struct ToFile;

impl ToFile {
    /// ## Парсит операции в зависимости от типа файла
    ///
    /// ### Arguments
    /// * `w` - writer
    /// * `operations` - операции
    /// * `file_type` - тип файла
    #[instrument(skip(w), name = "parse_operations_to_file")]
    pub fn operations<W: std::io::Write>(
        w: &mut W,
        operations: &[OperationName],
        file_type: FileType,
    ) -> Result<(), ParseFileError> {
        match file_type {
            FileType::BIN => bin::parse_to_bin(w, operations),
            FileType::CSV => csv::parse_to_csv(w, operations),
            FileType::TXT => txt::parse_to_txt(w, operations),
        }
        .inspect_err(|e| error!("ошибка при парсинге операций: {}", e))
    }
}
