mod bin;
mod csv;
mod txt;
use crate::{FileType, OperationName, errors::ParseFileError};

/// Парсит файл в зависимости от его типа
/// (пока только операции)
pub struct ToFile;

impl ToFile {
    pub fn operations<W: std::io::Write>(
        w: &mut W,
        operations: &Vec<OperationName>,
        file_type: FileType,
    ) -> Result<(), ParseFileError> {
        match file_type {
            FileType::BIN => bin::parse_to_bin(w, operations),
            FileType::CSV => csv::parse_to_csv(w, operations),
            FileType::TXT => txt::parse_to_txt(w, operations),
        }
    }
}
