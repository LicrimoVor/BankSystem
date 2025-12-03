use crate::{OperationName, errors::ParseFileError};
use bank::balance::operations::{Operation, OperationStatus, OperationType};

/// Сериализует операцию в бинарном формате
fn parse_body(body: &[u8]) -> Result<OperationName, ParseFileError> {
    let mut i = 0;
    let id = {
        let end = i + 8;
        let arr: [u8; 8] = body[i..end].try_into().expect("REASON");
        i = end;
        u64::from_be_bytes(arr)
    };
    let tx_type = {
        let res = match body[i] {
            0 => Ok(OperationType::Deposit(0)),
            1 => Ok(OperationType::Transfer("".to_string(), 0, false)),
            2 => Ok(OperationType::Withdraw(0)),
            _ => Err(()),
        };
        i += 1;

        res
    }
    .or(Err(ParseFileError::SerializeError("Неверный тип операции")))?;
    let from_user = {
        let end = i + 8;
        let arr: [u8; 8] = body[i..end].try_into().expect("REASON");
        i = end;
        u64::from_be_bytes(arr)
    };
    let to_user = {
        let end = i + 8;
        let arr: [u8; 8] = body[i..end].try_into().expect("REASON");
        i = end;
        u64::from_be_bytes(arr)
    };
    let amount = {
        let end = i + 8;
        let arr: [u8; 8] = body[i..end].try_into().expect("REASON");
        i = end;
        i64::from_be_bytes(arr)
    };
    let timestamp = {
        let end = i + 8;
        let arr: [u8; 8] = body[i..end].try_into().expect("REASON");
        i = end;
        u64::from_be_bytes(arr)
    };
    let status = {
        let res = match body[i] {
            0 => Ok(OperationStatus::SUCCESS),
            1 => Ok(OperationStatus::FAILURE),
            2 => Ok(OperationStatus::PENDING),
            _ => Err(()),
        };
        i += 1;
        res
    }
    .or(Err(ParseFileError::SerializeError(
        "Неверный статус операции",
    )))?;
    let desc_len = {
        let end = i + 4;
        let arr: [u8; 4] = body[i..end].try_into().expect("REASON");
        i = end;
        u32::from_be_bytes(arr)
    };
    let desc = {
        let end = i + desc_len as usize;
        let arr: Vec<u8> = body[i..end].to_vec();
        String::from_utf8(arr)
            .expect("REASON")
            .trim_matches('"')
            .to_string()
    };

    let (tx_type, name) = match tx_type {
        OperationType::Deposit(_) => (OperationType::Deposit(amount.abs() as u64), to_user),
        OperationType::Withdraw(_) => (OperationType::Withdraw(amount.abs() as u64), from_user),
        OperationType::Transfer(_, _, _) => (
            OperationType::Transfer(from_user.to_string(), amount.abs() as u64, true),
            to_user,
        ),
        OperationType::Close => (OperationType::Close, to_user),
    };
    let operation = Operation::load(id, timestamp, tx_type, status, Some(desc));
    Ok(OperationName(operation, name.to_string()))
}

/// Преобразование bin-файла в список операций
pub(super) fn parse_from_bin<R: std::io::Read>(
    r: &mut R,
) -> Result<Vec<OperationName>, ParseFileError> {
    let mut operations: Vec<OperationName> = Vec::new();
    let mut buf_magic = [0; 4];

    loop {
        if r.read_exact(&mut buf_magic).is_err() {
            break;
        };

        if str::from_utf8(&buf_magic).expect("REASON").to_string() != "YPBN" {
            return Err(ParseFileError::SerializeError("магический символ"));
        }

        let mut buf_size = [0; 4];
        r.read_exact(&mut buf_size)
            .or_else(|e| Err(ParseFileError::IoError(e)))?;
        let record_size = u32::from_be_bytes(buf_size);
        if record_size < 46 {
            return Err(ParseFileError::SerializeError("длина записи (min 46)"));
        }
        let mut buf_body = vec![0; record_size as usize];
        r.read_exact(&mut buf_body)
            .or_else(|e| Err(ParseFileError::IoError(e)))?;

        let arr: [u8; 4] = buf_body[42..46].try_into().expect("REASON");
        let desc_size = u32::from_be_bytes(arr);
        if record_size != 46 + desc_size {
            return Err(ParseFileError::SerializeError(
                "длина записи (46 + desc_size)",
            ));
        };
        let operation = parse_body(&buf_body)?;
        operations.push(operation);

        buf_magic = [0; 4];
    }

    Ok(operations)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};
    const PATH_TEST: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/test.bin");

    #[test]
    fn test_parse_from_bin_success() {
        let file = File::open(PATH_TEST).unwrap();
        let mut buf = BufReader::new(file);
        let res = parse_from_bin(&mut buf);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.len(), 1000);
    }
}
