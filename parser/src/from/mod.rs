mod bin;
mod csv;
mod errors;
mod txt;

use crate::{FileType, OperationName};

pub struct FromFile;

impl FromFile {
    pub fn operations<R: std::io::Read>(
        r: &mut R,
        file_type: FileType,
    ) -> Result<Vec<OperationName>, errors::ParseFromFileError> {
        match file_type {
            FileType::BIN => bin::parse_from_bin(r),
            FileType::CSV => csv::parse_from_csv(r),
            FileType::TXT => txt::parse_from_txt(r),
        }
    }
}
