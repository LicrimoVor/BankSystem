use crate::{OperationName, errors::ParseFileError, types::CsvRecord};
use bank::balance::operations::{Operation, OperationType};

/// Преобразование csv-файла в список операций
pub(super) fn parse_from_csv<R: std::io::Read>(
    r: &mut R,
) -> Result<Vec<OperationName>, ParseFileError> {
    let mut rdr = csv::Reader::from_reader(r);

    let operations: Vec<Result<OperationName, ParseFileError>> = rdr
        .deserialize::<CsvRecord>()
        .map(|row| {
            let Ok(row) = row else {
                return Err(ParseFileError::SerializeError("Не соответствует шаблону"));
            };
            let CsvRecord {
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
                _ => Err(ParseFileError::SerializeError(
                    "tx_type ожидается: [DEPOSIT, WITHDRAWAL, TRANSFER]",
                )),
            }?;
            let operation = Operation::load(TX_ID, TIMESTAMP, tx_type, STATUS, Some(DESCRIPTION));

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
