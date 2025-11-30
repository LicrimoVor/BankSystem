use super::errors::ParseFromFileError;
use bank::balance::{
    Balance,
    operations::{Operation, OperationStatus, OperationType},
};

pub fn parse_from_csv(path: &str) -> Result<Vec<Balance>, ParseFromFileError> {
    Ok(Vec::new())
}
