pub mod errors;
pub mod from;
pub mod to;
use bank::{Name, balance::operations::Operation};

#[derive(Debug, PartialEq, Clone)]
pub enum FileType {
    CSV,
    TXT,
    BIN,
}

#[derive(Debug, PartialEq, Clone)]
pub struct OperationName(Operation, Name);
