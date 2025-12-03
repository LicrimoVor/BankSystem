mod bin;
mod csv;
mod txt;

use crate::{FileType, OperationName, errors::ParseFileError};
use tracing::{error, info, instrument};

/// ## Парсит фаил в зависимости от его типа
/// (пока только операции)
pub struct FromFile;

impl FromFile {
    /// ## Парсит операции в зависимости от типа файла
    ///
    /// ### Arguments
    /// * `r` - reader
    /// * `file_type` - тип файла
    #[instrument(skip(r), name = "parse_operations_from_file")]
    pub fn operations<R: std::io::Read>(
        r: &mut R,
        file_type: FileType,
    ) -> Result<Vec<OperationName>, ParseFileError> {
        info!("Парсим операции: {:?}", file_type);
        match file_type {
            FileType::BIN => bin::parse_from_bin(r),
            FileType::CSV => csv::parse_from_csv(r),
            FileType::TXT => txt::parse_from_txt(r),
        }
        .inspect_err(|e| error!("ошибка при парсинге операций: {}", e))
    }
}
