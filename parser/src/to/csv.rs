use crate::{OperationName, errors::ParseFileError};
use bank::balance::operations::{OperationStatus, OperationType};

const HEADER: [&str; 8] = [
    "TX_ID",
    "TX_TYPE",
    "FROM_USER_ID",
    "TO_USER_ID",
    "AMOUNT",
    "TIMESTAMP",
    "STATUS",
    "DESCRIPTION",
];

pub(super) fn parse_to_csv<W: std::io::Write>(
    w: &mut W,
    operations: &[OperationName],
) -> Result<(), ParseFileError> {
    write!(w, "{}\n", HEADER.join(",")).or_else(|e| Err(ParseFileError::IoError(e)))?;

    for op in operations {
        let mut row: Vec<String> = Vec::new();
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

        // немного копи-паста
        row.push(op.id().to_string());
        row.push(tx_type.to_string());
        row.push(from_name.to_string());
        row.push(to_name.to_string());
        row.push(amount.to_string());
        row.push(op.timestamp().to_string());
        row.push(status.to_string());
        row.push(op.description.to_string());
        write!(w, "{}\n", row.join(",")).or_else(|e| Err(ParseFileError::IoError(e)))?;
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
        let answer = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
1000000000000000,DEPOSIT,0,9223372036854775807,100,1633036860000,FAILURE,\"Record number 1\"
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
        let res = parse_to_csv(&mut buf, &operations);
        assert!(res.is_ok());
        let bytes = buf.into_inner().unwrap();
        let csv = String::from_utf8(bytes).unwrap();
        assert_eq!(csv, answer);
    }
}
