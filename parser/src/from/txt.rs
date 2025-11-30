use crate::{OperationName, errors::ParseFileError};
use bank::balance::operations::{Operation, OperationStatus, OperationType};

fn get_atr(rows: &str, atr_name: &str) -> Option<String> {
    let Some(i_st) = rows.find(atr_name) else {
        return None;
    };

    let i_end = i_st + rows[i_st..].find('\n').unwrap_or(rows.len() - i_st);
    let Some((_, value)) = rows[i_st..i_end].split_once(":") else {
        return None;
    };

    Some(value.trim().to_string())
}

/// Преобразование txt-файла в список операций
pub fn parse_from_txt<R: std::io::Read>(r: &mut R) -> Result<Vec<OperationName>, ParseFileError> {
    let mut buf = String::new();
    r.read_to_string(&mut buf)
        .or_else(|e| Err(ParseFileError::IoError(e)))?;

    let data: Vec<&str> = buf.split("\n\n").collect();

    let mut operations: Vec<Result<OperationName, ParseFileError>> = data
        .iter()
        .map(|rows| {
            let len_rows = rows.split("\n").count();
            if len_rows != 9 {
                return Err(ParseFileError::ParseError("Неверный формат строки"));
            }

            let tx_type = get_atr(rows, "TX_TYPE")
                .ok_or(ParseFileError::ParseError("Неверный формат tx_type"))?;
            let to_user_id = get_atr(rows, "TO_USER_ID")
                .ok_or(ParseFileError::ParseError("Неверный формат to_user_id"))?;
            let from_user_id = get_atr(rows, "FROM_USER_ID")
                .ok_or(ParseFileError::ParseError("Неверный формат from_user_id"))?;
            let timestamp = get_atr(rows, "TIMESTAMP")
                .ok_or(ParseFileError::ParseError("Неверный формат timestamp"))?;
            let description = get_atr(rows, "DESCRIPTION")
                .ok_or(ParseFileError::ParseError("Неверный формат description"))?;
            let tx_id = get_atr(rows, "TX_ID")
                .ok_or(ParseFileError::ParseError("Неверный формат tx_id"))?;
            let amount = get_atr(rows, "AMOUNT")
                .ok_or(ParseFileError::ParseError("Неверный формат amount"))?;
            let status = get_atr(rows, "STATUS")
                .ok_or(ParseFileError::ParseError("Неверный формат status"))?;

            let timestamp = timestamp
                .parse::<u64>()
                .or(Err(ParseFileError::ParseError("Неверный формат timestamp")))?;
            let amount = amount
                .parse::<u64>()
                .or(Err(ParseFileError::ParseError("Неверный формат amount")))?;
            let tx_id = tx_id
                .parse::<u64>()
                .or(Err(ParseFileError::ParseError("Неверный формат tx_id")))?;

            let tx_type = match tx_type.as_str() {
                "DEPOSIT" => Ok(OperationType::Deposit(amount)),
                "WITHDRAWAL" => Ok(OperationType::Withdraw(amount)),
                "TRANSFER" => Ok(OperationType::Transfer(from_user_id.clone(), amount, true)),
                _ => Err(ParseFileError::ParseError("Неверный формат tx_type")),
            }?;

            let status = match status.as_str() {
                "PENDING" => Ok(OperationStatus::PENDING),
                "SUCCESS" => Ok(OperationStatus::SUCCESS),
                "FAILURE" => Ok(OperationStatus::FAILURE),
                _ => Err(ParseFileError::ParseError("Неверный формат status")),
            }?;

            let name = if from_user_id == "0" {
                to_user_id
            } else {
                from_user_id
            };
            let operation = Operation::load(tx_id, timestamp, tx_type, status, Some(description));

            Ok(OperationName(operation, name.to_string()))
        })
        .collect();
    if let Some(op) = operations.last() {
        if op.is_err() {
            operations.pop();
        }
    }
    operations.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};
    const PATH_TEST: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/test.txt");

    #[test]
    fn test_parse_from_txt_success() {
        let file = File::open(PATH_TEST).unwrap();
        let mut buf = BufReader::new(file);
        let res = parse_from_txt(&mut buf);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 1000);
    }
}
