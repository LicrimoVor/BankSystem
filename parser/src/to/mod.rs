mod bin;
mod csv;
mod errors;
mod txt;

use crate::FileType;
use bank::balance::Balance;

pub struct ToFile;

impl ToFile {
    pub fn balances(
        path: &str,
        balances: Vec<Balance>,
        file_type: FileType,
    ) -> Result<(), errors::ParseFromFileError> {
        match file_type {
            FileType::BIN => bin::parse_to_bin(path, balances),
            FileType::CSV => csv::parse_to_csv(path, balances),
            FileType::TXT => txt::parse_to_txt(path, balances),
        }
    }
}
