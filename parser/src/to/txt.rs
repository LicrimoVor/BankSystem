use crate::{OperationName, errors::ParseFileError};
use bank::balance::operations::{OperationStatus, OperationType};

pub(super) fn parse_to_txt<W: std::io::Write>(
    w: &mut W,
    operations: &[OperationName],
) -> Result<(), ParseFileError> {
    for (i, op) in operations.iter().enumerate() {
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
        let status = match &op.status {
            OperationStatus::SUCCESS => "SUCCESS",
            OperationStatus::FAILURE => "FAILURE",
            OperationStatus::PENDING => "PENDING",
        };
        let indx = i + 1;
        let id = op.id();
        let timestamp = op.timestamp();
        let description = &op.description;
        let data = format!(
            "# Record {indx} ({tx_type})
TX_ID: {id}
TX_TYPE: {tx_type}
FROM_USER_ID: {from_name}
TO_USER_ID: {to_name}
AMOUNT: {amount}
TIMESTAMP: {timestamp}
STATUS: {status}
DESCRIPTION: {description}",
        );
        write!(w, "{data}\n\n").or_else(|e| Err(ParseFileError::IoError(e)))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bank::balance::operations::Operation;
    use std::io::BufWriter;

    #[test]
    fn test_parse_to_bin_success() {
        let answer = "# Record 1 (DEPOSIT)
TX_ID: 1000000000000000
TX_TYPE: DEPOSIT
FROM_USER_ID: 0
TO_USER_ID: 9223372036854775807
AMOUNT: 100
TIMESTAMP: 1633036860000
STATUS: FAILURE
DESCRIPTION: \"Record number 1\"

";
        let operations = vec![OperationName(
            Operation::load(
                1000000000000000,
                1633036860000,
                OperationType::Deposit(100),
                OperationStatus::FAILURE,
                Some("\"Record number 1\"".to_string()),
            ),
            "9223372036854775807".to_string(),
        )];
        let mut buf = BufWriter::new(Vec::new());
        let res = parse_to_txt(&mut buf, &operations);
        assert!(res.is_ok());
        let bytes = buf.into_inner().unwrap();
        let csv = String::from_utf8(bytes).unwrap();
        assert_eq!(csv, answer);
    }
}
