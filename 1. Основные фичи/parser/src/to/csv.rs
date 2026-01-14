use crate::{OperationName, errors::ParseFileError, types::CsvRecord};
use bank::balance::operations::OperationType;
use csv::Writer;

pub(super) fn parse_to_csv<W: std::io::Write>(
    w: &mut W,
    operations: &[OperationName],
) -> Result<(), ParseFileError> {
    let mut w_csv = Writer::from_writer(w);

    for op in operations {
        let OperationName(op, name) = op;
        let (tx_type, amount, from_name, to_name) = match &op.tx_type {
            OperationType::Deposit(amount) => ("DEPOSIT", amount, "0", name.as_str()),
            OperationType::Withdraw(amount) => ("WITHDRAWAL", amount, name.as_str(), "0"),
            // будем учитывать операцию перевода только с
            // аккаунта-получателя
            OperationType::Transfer(from_name, amount, true) => {
                ("TRANSFER", amount, from_name.as_str(), name.as_str())
            }
            // пропускаем операции перевода с аккаунта-отправителя
            OperationType::Transfer(_, _, false) => continue,
            OperationType::Close => continue,
        };
        let from_name = from_name
            .parse::<u64>()
            .or(Err(ParseFileError::SerializeError(
                "Пользователь должен иметь ID",
            )))?;
        let to_name = to_name
            .parse::<u64>()
            .or(Err(ParseFileError::SerializeError(
                "Пользователь должен иметь ID",
            )))?;

        w_csv
            .serialize(CsvRecord {
                TX_ID: op.id(),
                TX_TYPE: tx_type.to_string(),
                FROM_USER_ID: from_name,
                TO_USER_ID: to_name,
                AMOUNT: *amount,
                TIMESTAMP: op.timestamp(),
                STATUS: op.status.clone(),
                DESCRIPTION: op.description.clone(),
            })
            .map_err(|e| match e.into_kind() {
                csv::ErrorKind::Io(e) => ParseFileError::IoError(e),
                _ => ParseFileError::SerializeError("Не соответствует шаблону"),
            })?;
    }

    w_csv.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bank::balance::operations::{Operation, OperationStatus};
    use std::io::BufWriter;

    #[test]
    fn test_parse_to_csv_success() {
        let answer = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
1000000000000000,DEPOSIT,0,9223372036854775807,100,1633036860000,FAILURE,Record number 1
";
        let operations = vec![OperationName(
            Operation::load(
                1000000000000000,
                1633036860000,
                OperationType::Deposit(100),
                OperationStatus::FAILURE,
                Some("Record number 1".to_string()),
            ),
            "9223372036854775807".to_string(),
        )];
        let mut buf = BufWriter::new(Vec::new());
        let res = parse_to_csv(&mut buf, &operations);
        assert!(res.is_ok());
        let bytes = buf.into_inner().unwrap();
        let csv = String::from_utf8(bytes).unwrap();
        assert_eq!(csv, answer);
    }
}
