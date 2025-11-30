use crate::{OperationName, errors::ParseFileError};
use bank::balance::operations::{Operation, OperationStatus, OperationType};
use serde::Deserialize;

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct Record {
    TX_ID: u64,
    TX_TYPE: String,
    FROM_USER_ID: u64,
    TO_USER_ID: u64,
    AMOUNT: u64,
    TIMESTAMP: u64,
    STATUS: String,
    DESCRIPTION: String,
}

/// Преобразование csv-файла в список операций
pub fn parse_from_csv<R: std::io::Read>(r: &mut R) -> Result<Vec<OperationName>, ParseFileError> {
    let mut rdr = csv::Reader::from_reader(r);

    let operations: Vec<Result<OperationName, ParseFileError>> = rdr
        .deserialize::<Record>()
        .map(|row| {
            let Ok(row) = row else {
                return Err(ParseFileError::ParseError("Неверный формат строки"));
            };
            let Record {
                TX_ID,
                TX_TYPE,
                FROM_USER_ID,
                TO_USER_ID,
                AMOUNT,
                TIMESTAMP,
                STATUS,
                DESCRIPTION,
            } = row;

            let (tx_type, name) = match TX_TYPE.as_str() {
                "DEPOSIT" => Ok((OperationType::Deposit(AMOUNT), TO_USER_ID)),
                "TRANSFER" => Ok((
                    OperationType::Transfer(FROM_USER_ID.to_string(), AMOUNT, true),
                    TO_USER_ID,
                )),
                "WITHDRAWAL" => Ok((OperationType::Withdraw(AMOUNT), FROM_USER_ID)),
                _ => Err(()),
            }
            .or(Err(ParseFileError::ParseError("Неверный тип операции")))?;

            let status = match STATUS.as_str() {
                "SUCCESS" => Ok(OperationStatus::SUCCESS),
                "FAILURE" => Ok(OperationStatus::FAILURE),
                "PENDING" => Ok(OperationStatus::PENDING),
                _ => Err(()),
            }
            .or(Err(ParseFileError::ParseError("Неверный статус операции")))?;
            let operation = Operation::load(
                TX_ID,
                TIMESTAMP,
                tx_type,
                status,
                Some(format!("\"{DESCRIPTION}\"")),
            );

            Ok(OperationName(operation, name.to_string()))
        })
        .collect();

    operations.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};
    const PATH_TEST: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/test.csv");

    #[test]
    fn test_parse_from_csv_success() {
        let file = File::open(PATH_TEST).unwrap();
        let mut buf = BufReader::new(file);
        let res = parse_from_csv(&mut buf);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.len(), 1000);
    }
}
