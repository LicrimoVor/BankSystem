use super::errors::ParseFromFileError;
use bank::balance::{
    Balance,
    operations::{Operation, OperationStatus, OperationType},
};

pub fn parse_to_csv(path: &str, balances: Vec<Balance>) -> Result<(), ParseFromFileError> {
    Ok(())
}
