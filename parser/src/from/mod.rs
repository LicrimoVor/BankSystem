mod bin;
mod csv;
mod errors;
mod txt;

use crate::FileType;
use bank::balance::operations::Operation;

pub struct FromFile;

// impl FromFile {
//     pub fn operations(
//         path: &str,
//         file_type: FileType,
//     ) -> Result<Vec<Operation>, errors::ParseFromFileError> {
//         match file_type {
//             FileType::BIN => bin::parse_from_bin(path),
//             FileType::CSV => csv::parse_from_csv(path),
//             FileType::TXT => txt::parse_from_txt(path),
//         }
//     }
// }
