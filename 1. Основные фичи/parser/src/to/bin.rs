use crate::{OperationName, errors::ParseFileError};
use bank::balance::operations::{OperationStatus, OperationType};

pub(super) fn parse_to_bin<W: std::io::Write>(
    w: &mut W,
    operations: &[OperationName],
) -> Result<(), ParseFileError> {
    for op in operations {
        w.write_all("YPBN".as_bytes())
            .map_err(ParseFileError::IoError)?;

        let mut body: Vec<u8> = vec![];
        let OperationName(op, name) = op;

        let (tx_type, amount, from_name, to_name) = match &op.tx_type {
            OperationType::Deposit(amount) => (0, amount, "0", name.as_str()),
            OperationType::Withdraw(amount) => (2, amount, name.as_str(), "0"),

            // будем учитывать операцию перевода только с
            // аккаунта-получателя
            OperationType::Transfer(from_name, amount, true) => {
                (1, amount, from_name.as_str(), name.as_str())
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
        let status: u8 = match &op.status {
            OperationStatus::SUCCESS => 0,
            OperationStatus::FAILURE => 1,
            OperationStatus::PENDING => 2,
        };
        body.extend_from_slice(&op.id().to_be_bytes());
        body.push(tx_type);
        body.extend_from_slice(&from_name.to_be_bytes());
        body.extend_from_slice(&to_name.to_be_bytes());
        body.extend_from_slice(&amount.to_be_bytes());
        body.extend_from_slice(&op.timestamp().to_be_bytes());
        body.push(status);

        let desc_str = format!("\"{}\"", op.description);
        let desc_bytes = desc_str.as_bytes();
        body.extend_from_slice(&(desc_bytes.len() as u32).to_be_bytes());
        body.extend_from_slice(desc_bytes);

        let body_len = body.len() as u32;
        w.write_all(&body_len.to_be_bytes())
            .map_err(ParseFileError::IoError)?;
        w.write_all(&body).map_err(ParseFileError::IoError)?;
    }
    w.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bank::balance::operations::Operation;
    use std::io::BufWriter;

    #[test]
    fn test_parse_to_bin_success() {
        let answer: Vec<u8> = vec![
            89, 80, 66, 78, 0, 0, 0, 63, 0, 3, 141, 126, 164, 198, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 127, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 100, 0, 0, 1, 124, 56,
            148, 250, 96, 1, 0, 0, 0, 17, 34, 82, 101, 99, 111, 114, 100, 32, 110, 117, 109, 98,
            101, 114, 32, 49, 34,
        ];
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
        let res = parse_to_bin(&mut buf, &operations);
        assert!(res.is_ok());
        assert_eq!(buf.into_inner().unwrap(), answer);
    }
}
