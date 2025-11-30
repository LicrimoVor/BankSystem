use crate::{OperationName, errors::ParseFileError};
use bank::balance::{
    Balance,
    operations::{Operation, OperationStatus, OperationType},
};

pub fn parse_to_csv<W: std::io::Write>(
    w: &mut W,
    operations: Vec<OperationName>,
) -> Result<(), ParseFileError> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};
    const PATH_TEST: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/test.bin");
}
